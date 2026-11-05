//! Pull-request hygiene (W-146): `git/commit-message`, `git/branch-name`,
//! `git/pr-size` and `git/pr-description`. They judge what git and the CI
//! already hold — the commits and size of the `--changed-since` range, the
//! branch, the pull request's title and description from the CI's own event
//! data — never a file, so each finding's `file` is a logical location
//! (`commit:<id>`, `branch:<name>`, `pr`; SR-ARCHITECTURE decision 14). In
//! the `commit-msg` hook and the agent hook a message is judged on its own.
//! Nothing is fetched (L5): what the CI does not put on disk or in the
//! environment is reported as not checked here.

use crate::git::{self, Commit};
use crate::Project;
use codelaw_core::{Diagnostic, Position, Severity};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;

pub const COMMIT_MESSAGE: &str = "git/commit-message";
pub const BRANCH_NAME: &str = "git/branch-name";
pub const PR_SIZE: &str = "git/pr-size";
pub const PR_DESCRIPTION: &str = "git/pr-description";

/// The environment variables the `git/*` rules read, and nothing else: the
/// branch and the pull request as each CI hands them to the job.
pub const CI_VARS: &[&str] = &[
    "GITHUB_EVENT_PATH",
    "GITHUB_HEAD_REF",
    "GITLAB_CI",
    "CI_MERGE_REQUEST_IID",
    "CI_MERGE_REQUEST_TITLE",
    "CI_MERGE_REQUEST_DESCRIPTION",
    "CI_MERGE_REQUEST_SOURCE_BRANCH_NAME",
    "CI_COMMIT_BRANCH",
    "BITBUCKET_BRANCH",
    "BITBUCKET_PR_ID",
    "SYSTEM_PULLREQUEST_PULLREQUESTID",
    "SYSTEM_PULLREQUEST_SOURCEBRANCH",
    "BUILD_SOURCEBRANCH",
];

/// The default `[commits] types` (the decider's list, 2026-10-30).
const CONVENTIONAL_TYPES: &[&str] = &["feat", "fix", "docs", "refactor", "test", "chore", "perf", "ci", "build"];
/// Angular's commit types.
const ANGULAR_TYPES: &[&str] = &["build", "ci", "docs", "feat", "fix", "perf", "refactor", "test"];
const DEFAULT_TICKET: &str = "[A-Z]+-[0-9]+";
const DEFAULT_SKIP_AUTHORS: &[&str] = &["dependabot[bot]", "renovate[bot]"];
const DEFAULT_PROTECTED: &[&str] = &["main", "develop", "release/*"];
const DEFAULT_PR_EXCLUDE: &[&str] = &["**/*.lock", "**/package-lock.json", "**/pnpm-lock.yaml", "**/generated/**"];

fn strs(v: Option<&Value>) -> Option<Vec<String>> {
    v?.as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_owned).collect())
}

fn owned(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// A table's options: the root config's section, the rule's own options
/// over it key by key (SR-CONFIG decision 11b).
fn merged(section: BTreeMap<String, Value>, rule: Option<&BTreeMap<String, Value>>) -> Map<String, Value> {
    let mut out: Map<String, Value> = section.into_iter().collect();
    for (k, v) in rule.into_iter().flatten() {
        match (out.get_mut(k), v) {
            (Some(Value::Object(a)), Value::Object(b)) => a.extend(b.clone()),
            _ => {
                out.insert(k.clone(), v.clone());
            }
        }
    }
    out
}

fn regex(key: &str, p: &str) -> Result<Regex, String> {
    Regex::new(p).map_err(|e| format!("`{key}`: `{p}` is not a regular expression: {e}"))
}

/// A whole-name match of `p`.
fn whole(key: &str, p: &str) -> Result<Regex, String> {
    regex(key, &format!("^(?:{p})$"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Preset {
    Conventional,
    Angular,
    Gitmoji,
    Custom,
    Off,
}

/// `[commits]`, resolved: what `git/commit-message` holds a message to.
#[derive(Clone, Debug)]
pub struct CommitRules {
    preset: Preset,
    types: Vec<String>,
    scopes: Vec<String>,
    require_scope: bool,
    subject_max: usize,
    subject_case: String,
    require_body: bool,
    breaking: Vec<String>,
    pattern: Option<Regex>,
    ticket_required: bool,
    ticket: Regex,
    ticket_place: String,
    skip_merges: bool,
    skip_reverts: bool,
    skip_fixups: bool,
    skip_authors: Vec<String>,
}

impl CommitRules {
    pub fn from_options(o: &Map<String, Value>) -> Result<CommitRules, String> {
        let preset = match o.get("preset").and_then(Value::as_str).unwrap_or("conventional") {
            "conventional" => Preset::Conventional,
            "angular" => Preset::Angular,
            "gitmoji" => Preset::Gitmoji,
            "custom" => Preset::Custom,
            "off" => Preset::Off,
            p => return Err(format!("`[commits] preset`: `{p}` is not one of `conventional`, `angular`, `gitmoji`, `custom`, `off`")),
        };
        let default_types = if preset == Preset::Angular { ANGULAR_TYPES } else { CONVENTIONAL_TYPES };
        let pattern = match o.get("pattern").and_then(Value::as_str).filter(|p| !p.is_empty()) {
            Some(p) => Some(regex("[commits] pattern", p)?),
            None if preset == Preset::Custom => return Err("`[commits] preset = \"custom\"` needs a `pattern`".into()),
            None => None,
        };
        let ticket = o.get("ticket").cloned().unwrap_or_default();
        let skip = o.get("skip").cloned().unwrap_or_default();
        let flag = |v: &Value, k: &str, d: bool| v.get(k).and_then(Value::as_bool).unwrap_or(d);
        Ok(CommitRules {
            preset,
            types: strs(o.get("types")).unwrap_or_else(|| owned(default_types)),
            scopes: strs(o.get("scopes")).unwrap_or_default(),
            require_scope: o.get("require_scope").and_then(Value::as_bool).unwrap_or(false),
            subject_max: o.get("subject_max").and_then(Value::as_u64).unwrap_or(72) as usize,
            subject_case: o.get("subject_case").and_then(Value::as_str).unwrap_or("lower").to_owned(),
            require_body: o.get("require_body").and_then(Value::as_bool).unwrap_or(false),
            breaking: strs(o.get("breaking")).unwrap_or_else(|| owned(&["!", "BREAKING CHANGE:"])),
            pattern,
            ticket_required: flag(&ticket, "required", false),
            ticket: regex("[commits.ticket] pattern", ticket.get("pattern").and_then(Value::as_str).unwrap_or(DEFAULT_TICKET))?,
            ticket_place: ticket.get("place").and_then(Value::as_str).unwrap_or("footer").to_owned(),
            skip_merges: flag(&skip, "merges", true),
            skip_reverts: flag(&skip, "reverts", true),
            skip_fixups: flag(&skip, "fixups", true),
            skip_authors: strs(skip.get("authors")).unwrap_or_else(|| owned(DEFAULT_SKIP_AUTHORS)),
        })
    }

    pub fn off(&self) -> bool {
        self.preset == Preset::Off
    }

    /// Why a commit is not judged: a merge, a revert, a fixup or a bot's.
    fn skipped(&self, header: &str, parents: usize, author: Option<(&str, &str)>) -> bool {
        (self.skip_merges && (parents > 1 || header.starts_with("Merge ")))
            || (self.skip_reverts && header.starts_with("Revert \""))
            || (self.skip_fixups && ["fixup! ", "squash! ", "amend! "].iter().any(|p| header.starts_with(p)))
            || author.is_some_and(|(n, e)| self.skip_authors.iter().any(|a| a == n || a == e))
    }

    /// What is wrong with `message`, each a short clause; empty when it
    /// passes. The message is already stripped of comment lines.
    pub fn problems(&self, message: &str) -> Vec<String> {
        let mut out = Vec::new();
        if self.off() {
            return out;
        }
        let mut lines = message.lines();
        let header = lines.next().unwrap_or("").trim_end();
        let rest: Vec<&str> = lines.collect();
        if header.trim().is_empty() {
            return vec!["the message is empty".into()];
        }
        let subject = match self.preset {
            Preset::Conventional | Preset::Angular => self.conventional(header, &mut out),
            Preset::Gitmoji => gitmoji(header).or_else(|| {
                out.push("the subject line does not start with a gitmoji (`:sparkles:` or ✨) and a space".into());
                None
            }),
            Preset::Custom => {
                if !self.pattern.as_ref().is_some_and(|p| p.is_match(header)) {
                    out.push(format!("the subject line does not match `[commits] pattern` `{}`", self.pattern.as_ref().map(Regex::as_str).unwrap_or_default()));
                }
                None
            }
            Preset::Off => None,
        };
        if let Some(s) = subject {
            if s.trim().is_empty() {
                out.push("the subject after the type is empty".into());
            } else if let Some(first) = s.chars().next() {
                match self.subject_case.as_str() {
                    "lower" if first.is_uppercase() => out.push("the subject starts with a capital letter; `[commits] subject_case` is `lower`".into()),
                    "sentence" if first.is_lowercase() => out.push("the subject starts with a lower-case letter; `[commits] subject_case` is `sentence`".into()),
                    _ => {}
                }
            }
        }
        let n = header.chars().count();
        if self.subject_max > 0 && n > self.subject_max {
            out.push(format!("the subject line is {n} characters, more than `[commits] subject_max` {}", self.subject_max));
        }
        if rest.first().is_some_and(|l| !l.trim().is_empty()) {
            out.push("the second line is not blank; leave one blank line between the subject and the body".into());
        }
        let body: Vec<&str> = rest.iter().copied().filter(|l| !l.trim().is_empty()).collect();
        if self.require_body && body.is_empty() {
            out.push("there is no body; `[commits] require_body` is on".into());
        }
        if self.ticket_required {
            let in_subject = self.ticket.is_match(header);
            let in_footer = body.iter().any(|l| self.ticket.is_match(l));
            let ok = match self.ticket_place.as_str() {
                "subject" => in_subject,
                "footer" => in_footer,
                _ => in_subject || in_footer,
            };
            if !ok {
                let place = match self.ticket_place.as_str() {
                    "subject" => "the subject line",
                    "footer" => "the body or footer",
                    _ => "the message",
                };
                out.push(format!("no ticket matching `{}` in {place}", self.ticket.as_str()));
            }
        }
        out
    }

    /// The subject of a Conventional Commit header, the problems with its
    /// type and scope pushed.
    fn conventional<'a>(&self, header: &'a str, out: &mut Vec<String>) -> Option<&'a str> {
        let Some(c) = conventional_re().captures(header) else {
            out.push("the subject line is not `<type>(<scope>): <subject>`".into());
            return None;
        };
        let ty = c.name("type").map(|m| m.as_str()).unwrap_or_default();
        if !self.types.iter().any(|t| t == ty) {
            out.push(format!("`{ty}` is not one of the commit types: {}", self.types.join(", ")));
        }
        match c.name("scope").map(|m| m.as_str()) {
            None if self.require_scope => out.push("there is no scope; `[commits] require_scope` is on".into()),
            Some(s) if !self.scopes.is_empty() && !self.scopes.iter().any(|x| x == s) => out.push(format!("`{s}` is not one of the scopes: {}", self.scopes.join(", "))),
            _ => {}
        }
        if c.name("bang").is_some() && !self.breaking.iter().any(|b| b == "!") {
            out.push("`!` does not mark a breaking change here; write a `BREAKING CHANGE:` footer".into());
        }
        Some(c.name("subject").map(|m| m.as_str()).unwrap_or_default())
    }

    /// A corrected subject line for `message`, when one can be derived.
    pub fn example(&self, message: &str) -> Option<String> {
        let header = message.lines().next()?.trim();
        let (ty, scope, subject) = match self.preset {
            Preset::Conventional | Preset::Angular => match conventional_re().captures(header) {
                Some(c) => (
                    c.name("type").map(|m| m.as_str().to_owned()).filter(|t| self.types.contains(t)),
                    c.name("scope").map(|m| m.as_str().to_owned()),
                    c.name("subject").map(|m| m.as_str()).unwrap_or_default().to_owned(),
                ),
                None => (None, None, header.trim_end_matches(['.', ' ']).to_owned()),
            },
            Preset::Gitmoji => (None, None, gitmoji(header).unwrap_or(header).to_owned()),
            _ => return None,
        };
        let subject = subject.trim().trim_end_matches('.').to_owned();
        let mut chars = subject.chars();
        let subject = match (chars.next(), self.subject_case.as_str()) {
            (Some(f), "lower") if f.is_uppercase() && !chars.clone().next().is_some_and(char::is_uppercase) => f.to_lowercase().chain(chars).collect(),
            (Some(f), "sentence") => f.to_uppercase().chain(chars).collect(),
            _ => subject,
        };
        let prefix = match self.preset {
            Preset::Gitmoji => ":sparkles: ".to_owned(),
            _ => {
                let ty = ty.unwrap_or_else(|| self.guess_type(&subject));
                let scope = match scope.filter(|s| self.scopes.is_empty() || self.scopes.contains(s)) {
                    Some(s) => format!("({s})"),
                    None if self.require_scope => format!("({})", self.scopes.first().map(String::as_str).unwrap_or("<scope>")),
                    None => String::new(),
                };
                format!("{ty}{scope}: ")
            }
        };
        let room = self.subject_max.saturating_sub(prefix.chars().count());
        let subject: String = if self.subject_max > 0 && subject.chars().count() > room { subject.chars().take(room).collect::<String>().trim_end().to_owned() } else { subject };
        let subject = if subject.is_empty() { "<what the commit does>".to_owned() } else { subject };
        Some(format!("{prefix}{subject}"))
    }

    fn guess_type(&self, subject: &str) -> String {
        let first = subject.split(|c: char| !c.is_alphanumeric()).next().unwrap_or_default().to_lowercase();
        let guess = match first.as_str() {
            "fix" | "fixed" | "fixes" | "fixing" | "bug" | "bugfix" | "hotfix" => "fix",
            "add" | "added" | "adds" | "adding" | "implement" | "implemented" | "new" | "support" | "introduce" => "feat",
            "doc" | "docs" | "document" | "documented" | "readme" => "docs",
            "refactor" | "refactored" | "rename" | "renamed" | "move" | "moved" | "extract" => "refactor",
            "test" | "tests" | "tested" => "test",
            "bump" | "update" | "updated" | "upgrade" | "upgraded" => "chore",
            _ => "",
        };
        if self.types.iter().any(|t| t == guess) {
            return guess.to_owned();
        }
        self.types.iter().find(|t| *t == "chore").or(self.types.first()).cloned().unwrap_or_else(|| "chore".into())
    }
}

fn conventional_re() -> &'static Regex {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(?P<type>[A-Za-z][\w-]*)(?:\((?P<scope>[^()\s]+)\))?(?P<bang>!)?: (?P<subject>.*)$").expect("regex"))
}

/// The subject after a leading gitmoji (`:sparkles:` or the emoji itself).
fn gitmoji(header: &str) -> Option<&str> {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^(?::[a-z0-9_+\-]+:|\p{Extended_Pictographic}[\x{FE0F}\x{200D}\p{Extended_Pictographic}]*)\s+(?P<subject>\S.*)$").expect("regex"));
    re.captures(header).and_then(|c| c.name("subject")).map(|m| m.as_str())
}

/// The message git would record from a `commit-msg` hook's file: comment
/// lines dropped, nothing after the scissors line, outer blank lines trimmed.
pub fn clean_message(text: &str) -> String {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with("# ------------------------ >8 ------------------------") {
            break;
        }
        if line.starts_with('#') {
            continue;
        }
        out.push(line.trim_end());
    }
    out.join("\n").trim_matches('\n').to_owned()
}

/// `[branches]`, resolved.
#[derive(Clone, Debug)]
struct BranchRules {
    pattern: Option<Regex>,
    max_length: usize,
    protected: globset::GlobSet,
    skip_authors: Vec<String>,
    ticket_required: bool,
    ticket: Regex,
    match_commits: bool,
}

impl BranchRules {
    fn from_options(o: &Map<String, Value>) -> Result<BranchRules, String> {
        let ticket = o.get("ticket").cloned().unwrap_or_default();
        let mut protected = globset::GlobSetBuilder::new();
        for g in strs(o.get("protected")).unwrap_or_else(|| owned(DEFAULT_PROTECTED)) {
            protected.add(globset::GlobBuilder::new(&g).literal_separator(true).build().map_err(|e| format!("`[branches] protected`: `{g}`: {e}"))?);
        }
        Ok(BranchRules {
            pattern: match o.get("pattern").and_then(Value::as_str).filter(|p| !p.is_empty()) {
                Some(p) => Some(whole("[branches] pattern", p)?),
                None => None,
            },
            max_length: o.get("max_length").and_then(Value::as_u64).unwrap_or(60) as usize,
            protected: protected.build().map_err(|e| e.to_string())?,
            skip_authors: strs(o.get("skip_authors")).unwrap_or_else(|| owned(DEFAULT_SKIP_AUTHORS)),
            ticket_required: ticket.get("required").and_then(Value::as_bool).unwrap_or(false),
            ticket: regex("[branches.ticket] pattern", ticket.get("pattern").and_then(Value::as_str).unwrap_or(DEFAULT_TICKET))?,
            match_commits: ticket.get("match_commits").and_then(Value::as_bool).unwrap_or(false),
        })
    }

    /// Branch names are checked only once a pattern is set or a ticket
    /// required (the agent's default, SR-CONFIG decision 11b).
    fn on(&self) -> bool {
        self.pattern.is_some() || self.ticket_required || self.match_commits
    }

    fn problems(&self, name: &str) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(p) = &self.pattern
            && !p.is_match(name)
        {
            out.push(format!("it does not match `[branches] pattern` `{}`", &p.as_str()[4..p.as_str().len() - 2]));
        }
        let n = name.chars().count();
        if self.max_length > 0 && n > self.max_length {
            out.push(format!("it is {n} characters, more than `[branches] max_length` {}", self.max_length));
        }
        if self.ticket_required && !self.ticket.is_match(name) {
            out.push(format!("it names no ticket matching `{}`", self.ticket.as_str()));
        }
        out
    }

    /// A name that passes, derived from `name`, when one can be found.
    fn example(&self, name: &str, types: &[String]) -> Option<String> {
        let mut slug = String::new();
        for c in name.rsplit('/').next().unwrap_or(name).chars() {
            if c.is_ascii_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
            } else if !slug.ends_with('-') && !slug.is_empty() {
                slug.push('-');
            }
        }
        let slug = slug.trim_end_matches('-').to_owned();
        let ticket = self.ticket.find(name).map(|m| m.as_str().to_owned()).unwrap_or_else(|| "ABC-123".into());
        let mut candidates = vec![slug.clone()];
        for t in types {
            candidates.push(format!("{t}/{slug}"));
            candidates.push(format!("{t}/{ticket}-{slug}"));
            candidates.push(format!("{t}/{}-{slug}", ticket.to_lowercase()));
        }
        candidates.push(format!("{ticket}-{slug}"));
        candidates.push(format!("{}-{slug}", ticket.to_lowercase()));
        candidates.into_iter().map(|c| if self.max_length > 0 { c.chars().take(self.max_length).collect::<String>().trim_end_matches('-').to_owned() } else { c }).find(|c| c != name && self.problems(c).is_empty())
    }
}

/// `[pr]`, resolved.
#[derive(Clone, Debug)]
struct PrRules {
    max_lines: u64,
    exclude: globset::GlobSet,
    title_pattern: Option<Regex>,
    require_ticket: bool,
    ticket: Regex,
    sections: Vec<String>,
}

impl PrRules {
    fn from_options(o: &Map<String, Value>, ticket: &Regex) -> Result<PrRules, String> {
        let mut exclude = globset::GlobSetBuilder::new();
        for g in strs(o.get("exclude")).unwrap_or_else(|| owned(DEFAULT_PR_EXCLUDE)) {
            exclude.add(globset::Glob::new(&g).map_err(|e| format!("`[pr] exclude`: `{g}`: {e}"))?);
            // `**/x` matches `x` at the top too, as `.gitignore` does.
            if let Some(top) = g.strip_prefix("**/") {
                exclude.add(globset::Glob::new(top).map_err(|e| format!("`[pr] exclude`: `{g}`: {e}"))?);
            }
        }
        Ok(PrRules {
            max_lines: o.get("max_lines").and_then(Value::as_u64).unwrap_or(400),
            exclude: exclude.build().map_err(|e| e.to_string())?,
            title_pattern: match o.get("title_pattern").and_then(Value::as_str).filter(|p| !p.is_empty()) {
                Some(p) => Some(regex("[pr] title_pattern", p)?),
                None => None,
            },
            require_ticket: o.get("require_ticket").and_then(Value::as_bool).unwrap_or(false),
            ticket: ticket.clone(),
            sections: strs(o.get("template_sections")).unwrap_or_default(),
        })
    }

    /// The description is checked only once a title pattern, a ticket or
    /// a section is asked for (SR-CONFIG decision 11b).
    fn description_on(&self) -> bool {
        self.title_pattern.is_some() || self.require_ticket || !self.sections.is_empty()
    }

    fn description_problems(&self, pr: &PullRequest) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(p) = &self.title_pattern
            && !p.is_match(&pr.title)
        {
            out.push(format!("the title does not match `[pr] title_pattern` `{}`", p.as_str()));
        }
        if self.require_ticket {
            match &pr.body {
                Some(b) if self.ticket.is_match(&pr.title) || self.ticket.is_match(b) => {}
                None if self.ticket.is_match(&pr.title) => {}
                Some(_) => out.push(format!("neither the title nor the description names a ticket matching `{}`", self.ticket.as_str())),
                None => out.push(format!("the title names no ticket matching `{}`", self.ticket.as_str())),
            }
        }
        if let Some(body) = &pr.body {
            for s in &self.sections {
                match section_text(body, s) {
                    None => out.push(format!("the description has no `{s}` section")),
                    Some(t) if t.trim().is_empty() => out.push(format!("the `{s}` section is empty")),
                    Some(_) => {}
                }
            }
        }
        out
    }
}

/// The text under the Markdown heading `name` (any level, case ignored),
/// HTML comments removed, up to the next heading of the same or a higher
/// level; `None` when there is no such heading.
pub fn section_text(body: &str, name: &str) -> Option<String> {
    let heading = |l: &str| -> Option<(usize, String)> {
        let t = l.trim_start();
        let level = t.chars().take_while(|c| *c == '#').count();
        (level > 0 && level <= 6 && t[level..].starts_with([' ', '\t'])).then(|| (level, t[level..].trim().trim_end_matches('#').trim().to_lowercase()))
    };
    let want = name.trim().trim_start_matches('#').trim().to_lowercase();
    let mut lines = body.lines();
    let level = loop {
        let l = lines.next()?;
        if let Some((lv, h)) = heading(l)
            && h == want
        {
            break lv;
        }
    };
    let mut text = String::new();
    for l in lines {
        if heading(l).is_some_and(|(lv, _)| lv <= level) {
            break;
        }
        text.push_str(l);
        text.push('\n');
    }
    // `<!-- … -->` is the template's prompt, not an answer.
    let mut out = String::new();
    let mut rest = text.as_str();
    while let Some(i) = rest.find("<!--") {
        out.push_str(&rest[..i]);
        rest = rest[i..].find("-->").map(|j| &rest[i + j + 3..]).unwrap_or("");
    }
    out.push_str(rest);
    Some(out)
}

/// A pull request's title and description, as the CI handed them over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PullRequest {
    pub title: String,
    /// `None` where the CI gives the title alone.
    pub body: Option<String>,
}

/// What the CI's environment says about the pull request (DoD 4): GitHub's
/// event file, GitLab's variables. Bitbucket and Azure DevOps put no title
/// or description in the job without an API call, which is never made.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrData {
    Found(PullRequest),
    /// A pull-request job on a CI that does not hand over the description.
    NotHere(&'static str),
    /// Not a pull-request job.
    None,
}

pub fn pr_data(vars: &BTreeMap<String, String>) -> PrData {
    let var = |k: &str| vars.get(k).filter(|v| !v.is_empty());
    if let Some(path) = var("GITHUB_EVENT_PATH") {
        let event: Option<Value> = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str(&t).ok());
        if let Some(pr) = event.as_ref().and_then(|e| e.get("pull_request")) {
            return PrData::Found(PullRequest {
                title: pr.get("title").and_then(Value::as_str).unwrap_or_default().to_owned(),
                body: Some(pr.get("body").and_then(Value::as_str).unwrap_or_default().to_owned()),
            });
        }
        return PrData::None;
    }
    if let Some(title) = var("CI_MERGE_REQUEST_TITLE") {
        return PrData::Found(PullRequest { title: title.clone(), body: Some(var("CI_MERGE_REQUEST_DESCRIPTION").cloned().unwrap_or_default()) });
    }
    if var("BITBUCKET_PR_ID").is_some() {
        return PrData::NotHere("Bitbucket Pipelines");
    }
    if var("SYSTEM_PULLREQUEST_PULLREQUESTID").is_some() {
        return PrData::NotHere("Azure Pipelines");
    }
    PrData::None
}

/// The branch the CI built, before git's own answer.
pub fn ci_branch(vars: &BTreeMap<String, String>) -> Option<String> {
    let var = |k: &str| vars.get(k).filter(|v| !v.is_empty()).cloned();
    var("GITHUB_HEAD_REF")
        .or_else(|| var("CI_MERGE_REQUEST_SOURCE_BRANCH_NAME"))
        .or_else(|| var("CI_COMMIT_BRANCH"))
        .or_else(|| var("BITBUCKET_BRANCH"))
        .or_else(|| var("SYSTEM_PULLREQUEST_SOURCEBRANCH"))
        .or_else(|| var("BUILD_SOURCEBRANCH"))
        .map(|b| b.strip_prefix("refs/heads/").map(str::to_owned).unwrap_or(b))
}

fn finding(rule: &str, severity: Severity, file: String, message: String, data: Value) -> Diagnostic {
    let r = codelaw_rules::engine_rule(rule).expect("a git rule is an engine rule");
    let pos = Position { line: 1, column: 1 };
    Diagnostic {
        rule: rule.to_owned(),
        severity,
        file,
        start_byte: 0,
        end_byte: 0,
        start: pos,
        end: pos,
        message,
        fix: r.fix,
        edits: Vec::new(),
        steps: r.steps.iter().map(|s| s.to_string()).collect(),
        source: "engine".into(),
        note: None,
        data: Some(data),
        choices: Vec::new(),
        fix_exception: None,
    }
}

fn quote(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_owned() } else { format!("{}…", s.chars().take(max).collect::<String>()) }
}

impl Project {
    /// A `git/*` rule's severity for this run (`--rule` and the root's
    /// layers applied); `None` is off.
    fn git_rule(&self, id: &str) -> Option<Severity> {
        if self.options.rules.as_ref().is_some_and(|o| !o.iter().any(|x| x == id)) {
            return None;
        }
        crate::lint::meta_severity(&self.config.layers_for(&self.config.root_dir), id)
    }

    /// The rule's options from the root's layers, the last one that sets them.
    fn git_rule_options(&self, id: &str) -> Option<BTreeMap<String, Value>> {
        self.config.layers_for(&self.config.root_dir).iter().rev().find_map(|l| l.rules.get(id).and_then(|s| s.options()).filter(|o| !o.is_empty()).cloned())
    }

    pub fn commit_rules(&self) -> Result<CommitRules, String> {
        CommitRules::from_options(&merged(self.config.section_options("commits"), self.git_rule_options(COMMIT_MESSAGE).as_ref()))
    }

    /// `git/commit-message` over one message on its own: the `commit-msg`
    /// hook's file and the agent hook's `git commit -m`. `file` is where
    /// the finding points. `Err` is a configuration the rule cannot use.
    pub fn check_commit_message(&self, file: &str, text: &str) -> Result<Vec<Diagnostic>, String> {
        let Some(severity) = self.git_rule(COMMIT_MESSAGE) else { return Ok(Vec::new()) };
        let rules = self.commit_rules()?;
        let message = clean_message(text);
        let header = message.lines().next().unwrap_or("");
        if rules.off() || rules.skipped(header, 1, None) {
            return Ok(Vec::new());
        }
        Ok(commit_finding(&rules, severity, file.to_owned(), None, &message).into_iter().collect())
    }

    /// The `git/*` findings of a `--changed-since` run (W-146): each rule
    /// that is on, over the range's commits, the branch, the size and the
    /// pull request. Nothing runs without `--changed-since`.
    pub(crate) fn hygiene_findings(&self, warnings: &mut Vec<String>) -> Vec<Diagnostic> {
        let Some(reference) = self.options.changed_since.as_deref() else { return Vec::new() };
        let root = self.root();
        let mut out = Vec::new();
        let commit_rules = match self.commit_rules() {
            Ok(r) => Some(r),
            Err(e) => {
                warnings.push(format!("{COMMIT_MESSAGE}: {e}"));
                None
            }
        };
        let types = commit_rules.as_ref().map(|r| r.types.clone()).unwrap_or_default();
        let branch_rules = BranchRules::from_options(&merged(self.config.section_options("branches"), self.git_rule_options(BRANCH_NAME).as_ref()));
        let needs_commits = self.git_rule(COMMIT_MESSAGE).is_some() && commit_rules.as_ref().is_some_and(|r| !r.off())
            || self.git_rule(BRANCH_NAME).is_some() && branch_rules.as_ref().is_ok_and(|b| b.match_commits);
        let commits = if needs_commits {
            match git::commits_since(root, reference) {
                Ok(c) => c,
                Err(e) => {
                    warnings.push(format!("the commits since `{reference}` could not be read ({e}); `{COMMIT_MESSAGE}` checks nothing"));
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };
        if let (Some(severity), Some(rules)) = (self.git_rule(COMMIT_MESSAGE), &commit_rules) {
            for c in &commits {
                let header = c.message.lines().next().unwrap_or("");
                if rules.skipped(header, c.parents, Some((&c.author_name, &c.author_email))) {
                    continue;
                }
                let short = c.id.get(..7).unwrap_or(&c.id);
                out.extend(commit_finding(rules, severity, format!("commit:{short}"), Some(c), c.message.trim_end()));
            }
        }
        if let Some(severity) = self.git_rule(BRANCH_NAME) {
            match &branch_rules {
                Err(e) => warnings.push(format!("{BRANCH_NAME}: {e}")),
                Ok(b) if b.on() => out.extend(self.branch_finding(b, severity, &commits, commit_rules.as_ref(), &types, warnings)),
                Ok(_) => {}
            }
        }
        let ticket = commit_rules.as_ref().map(|r| r.ticket.clone()).unwrap_or_else(|| Regex::new(DEFAULT_TICKET).expect("regex"));
        let size = self.git_rule(PR_SIZE);
        let description = self.git_rule(PR_DESCRIPTION);
        if size.is_some() || description.is_some() {
            match PrRules::from_options(&merged(self.config.section_options("pr"), None), &ticket) {
                Err(e) => warnings.push(format!("[pr]: {e}")),
                Ok(pr) => {
                    if let Some(severity) = size.filter(|_| pr.max_lines > 0) {
                        match git::numstat_since(root, reference) {
                            Ok(stat) => out.extend(size_finding(&pr, severity, &stat)),
                            Err(e) => warnings.push(format!("the size of the change since `{reference}` could not be read ({e}); `{PR_SIZE}` checks nothing")),
                        }
                    }
                    if let Some(severity) = description.filter(|_| pr.description_on()) {
                        match pr_data(&self.options.ci_vars) {
                            PrData::Found(p) => {
                                let problems = pr.description_problems(&p);
                                if !problems.is_empty() {
                                    let msg = format!("pull request \"{}\": {}", quote(&p.title, 72), problems.join("; "));
                                    out.push(finding(PR_DESCRIPTION, severity, "pr".into(), msg, json!({ "title": p.title, "problems": problems })));
                                }
                            }
                            PrData::NotHere(ci) => warnings.push(format!(
                                "{ci} hands a job no pull-request title or description without an API call, which codelaw never makes; `{PR_DESCRIPTION}` is not checked here"
                            )),
                            PrData::None => warnings.push(format!("no pull-request data in this environment (GitHub's event file or GitLab's merge-request variables); `{PR_DESCRIPTION}` is not checked here")),
                        }
                    }
                }
            }
        }
        out
    }

    fn branch_finding(&self, b: &BranchRules, severity: Severity, commits: &[Commit], commit_rules: Option<&CommitRules>, types: &[String], warnings: &mut Vec<String>) -> Option<Diagnostic> {
        let root = self.root();
        let name = match ci_branch(&self.options.ci_vars) {
            Some(n) => n,
            None => match git::branch(root) {
                Ok(Some(n)) => n,
                Ok(None) => {
                    warnings.push(format!("`HEAD` is detached and the CI names no branch; `{BRANCH_NAME}` is not checked here"));
                    return None;
                }
                Err(e) => {
                    warnings.push(format!("the branch could not be read ({e}); `{BRANCH_NAME}` checks nothing"));
                    return None;
                }
            },
        };
        if b.protected.is_match(&name) {
            return None;
        }
        if let Ok((n, e)) = git::head_author(root)
            && b.skip_authors.iter().any(|a| *a == n || *a == e)
        {
            return None;
        }
        let mut problems = b.problems(&name);
        if b.match_commits {
            match b.ticket.find(&name) {
                Some(t) => {
                    let missing: Vec<String> = commits
                        .iter()
                        .filter(|c| !commit_rules.is_some_and(|r| r.skipped(c.message.lines().next().unwrap_or(""), c.parents, Some((&c.author_name, &c.author_email)))))
                        .filter(|c| !c.message.contains(t.as_str()))
                        .map(|c| c.id.get(..7).unwrap_or(&c.id).to_owned())
                        .collect();
                    if !missing.is_empty() {
                        problems.push(format!("its ticket `{}` is not named in commit {}", t.as_str(), missing.join(", ")));
                    }
                }
                None if !b.ticket_required => problems.push(format!("it names no ticket matching `{}` for its commits to name", b.ticket.as_str())),
                None => {}
            }
        }
        if problems.is_empty() {
            return None;
        }
        let example = b.example(&name, types);
        let mut msg = format!("branch \"{name}\": {}", problems.join("; "));
        if let Some(e) = &example {
            msg.push_str(&format!(". A name that passes: \"{e}\""));
        }
        Some(finding(BRANCH_NAME, severity, format!("branch:{name}"), msg, json!({ "branch": name, "problems": problems, "example": example })))
    }
}

/// One commit's finding: the subject it has, what is wrong, a subject that
/// passes. `None` when the message passes.
fn commit_finding(rules: &CommitRules, severity: Severity, file: String, commit: Option<&Commit>, message: &str) -> Option<Diagnostic> {
    let problems = rules.problems(message);
    if problems.is_empty() {
        return None;
    }
    let header = message.lines().next().unwrap_or("");
    let example = rules.example(message);
    let what = match commit {
        Some(c) => format!("commit {} \"{}\"", c.id.get(..7).unwrap_or(&c.id), quote(header, 80)),
        None => format!("commit message \"{}\"", quote(header, 80)),
    };
    let mut msg = format!("{what}: {}", problems.join("; "));
    if let Some(e) = &example {
        msg.push_str(&format!(". Write it as: \"{e}\""));
    }
    Some(finding(
        COMMIT_MESSAGE,
        severity,
        file,
        msg,
        json!({ "commit": commit.map(|c| c.id.clone()), "subject": header, "problems": problems, "example": example }),
    ))
}

fn size_finding(pr: &PrRules, severity: Severity, stat: &[(String, u64, u64)]) -> Option<Diagnostic> {
    let counted: Vec<&(String, u64, u64)> = stat.iter().filter(|(p, _, _)| !pr.exclude.is_match(p)).collect();
    let added: u64 = counted.iter().map(|(_, a, _)| a).sum();
    let deleted: u64 = counted.iter().map(|(_, _, d)| d).sum();
    let lines = added + deleted;
    if lines <= pr.max_lines {
        return None;
    }
    let excluded = stat.len() - counted.len();
    let msg = format!(
        "the change is {lines} lines ({added} added, {deleted} deleted) in {} files, more than `[pr] max_lines` {}{}",
        counted.len(),
        pr.max_lines,
        if excluded > 0 { format!("; {excluded} lockfile or generated files not counted") } else { String::new() }
    );
    Some(finding(PR_SIZE, severity, "pr".into(), msg, json!({ "lines": lines, "max": pr.max_lines, "added": added, "deleted": deleted, "files": counted.len(), "excluded": excluded })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(o: Value) -> CommitRules {
        CommitRules::from_options(o.as_object().unwrap()).unwrap()
    }

    #[test]
    fn conventional_by_default() {
        let r = rules(json!({}));
        assert!(r.problems("feat: add login").is_empty());
        assert!(r.problems("fix(api)!: drop v1\n\nBREAKING CHANGE: v1 is gone").is_empty());
        assert_eq!(r.problems("Added login"), ["the subject line is not `<type>(<scope>): <subject>`"]);
        assert_eq!(r.example("Added login page.").as_deref(), Some("feat: added login page"));
        assert_eq!(r.example("Fix the crash").as_deref(), Some("fix: fix the crash"));
        let p = r.problems("feature: Add login\nmore");
        assert_eq!(p.len(), 3, "{p:?}");
        assert!(p[0].starts_with("`feature` is not one of the commit types"));
        assert!(r.problems(&format!("feat: {}", "x".repeat(70))).iter().any(|p| p.contains("more than `[commits] subject_max` 72")));
    }

    #[test]
    fn every_preset() {
        let a = rules(json!({ "preset": "angular" }));
        assert!(a.problems("chore: bump").iter().any(|p| p.starts_with("`chore` is not one")));
        assert!(a.problems("build: bump").is_empty());
        let g = rules(json!({ "preset": "gitmoji" }));
        assert!(g.problems(":sparkles: add login").is_empty());
        assert!(g.problems("✨ add login").is_empty());
        assert!(!g.problems("add login").is_empty());
        assert_eq!(g.example("Add login").as_deref(), Some(":sparkles: add login"));
        let c = rules(json!({ "preset": "custom", "pattern": "^\\[[A-Z]+-\\d+\\] .+" }));
        assert!(c.problems("[ABC-1] Add login").is_empty());
        assert!(!c.problems("Add login").is_empty());
        assert!(CommitRules::from_options(json!({ "preset": "custom" }).as_object().unwrap()).is_err());
        assert!(rules(json!({ "preset": "off" })).problems("whatever").is_empty());
    }

    #[test]
    fn options_scope_case_body_ticket() {
        let r = rules(json!({ "scopes": ["api", "ui"], "require_scope": true, "subject_case": "sentence", "require_body": true, "ticket": { "required": true, "place": "footer" } }));
        let p = r.problems("feat: add login");
        assert!(p.iter().any(|x| x.contains("no scope")));
        assert!(p.iter().any(|x| x.contains("lower-case")));
        assert!(p.iter().any(|x| x.contains("no body")));
        assert!(p.iter().any(|x| x.contains("no ticket")));
        assert_eq!(r.example("feat: add login").as_deref(), Some("feat(api): Add login"));
        assert!(r.problems("feat(api): Add login\n\nWhy.\n\nRefs: ABC-12").is_empty());
        assert!(r.problems("feat(db): Add login\n\nWhy.\n\nRefs: ABC-12").iter().any(|x| x.contains("`db` is not one of the scopes")));
        let bang = rules(json!({ "breaking": ["BREAKING CHANGE:"] }));
        assert!(bang.problems("feat!: drop v1").iter().any(|x| x.contains("`!` does not mark")));
    }

    #[test]
    fn skips() {
        let r = rules(json!({}));
        assert!(r.skipped("Merge branch 'x'", 2, None));
        assert!(r.skipped("Revert \"feat: x\"", 1, None));
        assert!(r.skipped("fixup! feat: x", 1, None));
        assert!(r.skipped("Bump x from 1 to 2", 1, Some(("dependabot[bot]", "x@users.noreply.github.com"))));
        assert!(!r.skipped("Bump x", 1, Some(("Ann", "ann@x.org"))));
        let none = rules(json!({ "skip": { "merges": false, "reverts": false, "fixups": false, "authors": [] } }));
        assert!(!none.skipped("Merge branch 'x'", 2, Some(("dependabot[bot]", ""))));
    }

    #[test]
    fn a_hook_file_is_cleaned_as_git_cleans_it() {
        let t = "feat: x\n\nbody\n# Please enter the commit message\n# ------------------------ >8 ------------------------\ndiff --git a b\n";
        assert_eq!(clean_message(t), "feat: x\n\nbody");
    }

    #[test]
    fn branches() {
        let b = BranchRules::from_options(json!({ "pattern": "(feat|fix)/[a-z0-9-]+", "ticket": { "required": false } }).as_object().unwrap()).unwrap();
        assert!(b.on());
        assert!(b.problems("feat/login-page").is_empty());
        assert_eq!(b.problems("Login_Page").len(), 1);
        assert_eq!(b.example("Login_Page", &owned(CONVENTIONAL_TYPES)).as_deref(), Some("feat/login-page"));
        assert!(b.protected.is_match("main") && b.protected.is_match("release/1.2") && !b.protected.is_match("release/1/2"));
        let t = BranchRules::from_options(json!({ "ticket": { "required": true } }).as_object().unwrap()).unwrap();
        assert!(t.problems("login").iter().any(|p| p.contains("no ticket")));
        assert!(t.problems("ABC-12-login").is_empty());
        assert!(!BranchRules::from_options(&Map::new()).unwrap().on(), "unchecked until a pattern is set");
    }

    #[test]
    fn pr_size_excludes_lockfiles() {
        let pr = PrRules::from_options(&Map::new(), &Regex::new(DEFAULT_TICKET).unwrap()).unwrap();
        let stat = vec![("src/a.ts".to_owned(), 300, 50), ("package-lock.json".to_owned(), 5000, 10), ("x/generated/y.ts".to_owned(), 900, 0)];
        assert!(size_finding(&pr, Severity::Warn, &stat).is_none(), "350 lines counted");
        let stat = vec![("src/a.ts".to_owned(), 300, 101)];
        let d = size_finding(&pr, Severity::Warn, &stat).unwrap();
        assert_eq!(d.data.unwrap()["lines"], 401);
    }

    #[test]
    fn pr_description() {
        let pr = PrRules::from_options(json!({ "title_pattern": "^[A-Z]", "require_ticket": true, "template_sections": ["Summary", "Testing"] }).as_object().unwrap(), &Regex::new(DEFAULT_TICKET).unwrap()).unwrap();
        let good = PullRequest { title: "Add login".into(), body: Some("## Summary\nAdds it. ABC-1\n\n## Testing\n- unit\n".into()) };
        assert!(pr.description_problems(&good).is_empty());
        let bad = PullRequest { title: "add login".into(), body: Some("## Summary\n<!-- what and why -->\n\n## Notes\nx".into()) };
        let p = pr.description_problems(&bad);
        assert_eq!(p.len(), 4, "{p:?}");
        assert_eq!(section_text("# A\n### Testing ###\nok\n## Next\n", "testing").as_deref(), Some("ok\n"));
    }

    #[test]
    fn each_ci_hands_over_what_it_has() {
        let dir = std::env::temp_dir().join(format!("codelaw-hygiene-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let event = dir.join("event.json");
        std::fs::write(&event, r#"{"pull_request":{"title":"feat: x","body":null}}"#).unwrap();
        let vars = |kv: &[(&str, &str)]| kv.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<BTreeMap<_, _>>();
        assert_eq!(pr_data(&vars(&[("GITHUB_EVENT_PATH", event.to_str().unwrap())])), PrData::Found(PullRequest { title: "feat: x".into(), body: Some(String::new()) }));
        assert_eq!(
            pr_data(&vars(&[("CI_MERGE_REQUEST_TITLE", "t"), ("CI_MERGE_REQUEST_DESCRIPTION", "d")])),
            PrData::Found(PullRequest { title: "t".into(), body: Some("d".into()) })
        );
        assert_eq!(pr_data(&vars(&[("BITBUCKET_PR_ID", "7")])), PrData::NotHere("Bitbucket Pipelines"));
        assert_eq!(pr_data(&vars(&[("SYSTEM_PULLREQUEST_PULLREQUESTID", "7")])), PrData::NotHere("Azure Pipelines"));
        assert_eq!(pr_data(&vars(&[])), PrData::None);
        assert_eq!(ci_branch(&vars(&[("SYSTEM_PULLREQUEST_SOURCEBRANCH", "refs/heads/feat/x")])).as_deref(), Some("feat/x"));
        assert_eq!(ci_branch(&vars(&[("GITHUB_HEAD_REF", "fix/y"), ("CI_COMMIT_BRANCH", "z")])).as_deref(), Some("fix/y"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
