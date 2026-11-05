//! `.codelawignore` for every rule (SR-BASELINE decision 12; W-71, widened
//! by W-158): the file is read once at load, its rule ids are checked
//! against the rule set, and every report — `check`, `fix`, a watch, MCP
//! `check_snippet` — drops the findings a live line exempts, after every
//! rule has run and before the baseline. An edit revokes a line unless it
//! is `forever` or `on_touch=keep`. The test rules of new code are judged
//! in `new_code.rs`, which reads the same lines.
//!
//! The editor shows an exempted finding as it shows one the baseline holds
//! (SR-EDITOR decision 8): `check_document` keeps it, and the language
//! server asks [`Project::codelawignore_holds`]. `codelaw fix` asks it too,
//! and applies no fix to an exempted finding.

use crate::project::Source;
use crate::{EngineError, Project};
use codelaw_baseline::exception::{EXPIRED_RULE, EXPIRING_RULE, Expiry, expiry};
use codelaw_baseline::ignore_file::{self, IGNORE_FILE, INCOMPLETE_RULE, IgnoreEntry, IgnoreOnTouch, IgnorePolicy, IgnoreTarget, STALE_IGNORE_RULE, TouchScope};
use codelaw_core::{Diagnostic, FixClass, Language, LineIndex, Severity, TextEdit};
use codelaw_lang::{Change, Function};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// `.codelawignore` and its policy, read at load.
#[derive(Clone, Debug)]
pub(crate) struct IgnoreFile {
    pub policy: IgnorePolicy,
    pub text: String,
    pub entries: Vec<IgnoreEntry>,
}

fn config_error(file: Option<&Path>, e: String) -> EngineError {
    EngineError::Config(codelaw_config::ConfigError::new(file.map(Path::to_path_buf), e))
}

/// `[codelawignore]`, else its old spelling `[coverage.ignore]`.
pub(crate) fn policy(config: &codelaw_config::Config) -> Result<IgnorePolicy, String> {
    let raw = config.files.first().map(|f| &f.raw);
    let new = raw.and_then(|r| r.codelawignore.as_ref()).map(|i| ("codelawignore", i));
    let old = raw.and_then(|r| r.coverage.as_ref()).and_then(|c| c.ignore.as_ref()).map(|i| ("coverage.ignore", i));
    match new.or(old) {
        Some((table, i)) => IgnorePolicy::from_table(table, i.require_expiry, i.require_on_touch, i.on_touch.as_deref(), i.on_touch_scope.as_deref()),
        None => Ok(IgnorePolicy::default()),
    }
}

/// Whether `id` is a rule this project knows: a loaded rule or one of the
/// engine's.
fn known(rules: &codelaw_rules::RuleSet, id: &str) -> bool {
    rules.get(id).is_some() || codelaw_rules::engine_rule(id).is_some()
}

/// Whether namespace `ns` (no `/*`) holds any rule this project knows.
fn known_namespace(rules: &codelaw_rules::RuleSet, ns: &str) -> bool {
    let prefix = format!("{ns}/");
    rules.rules.iter().any(|r| r.rule.id.starts_with(&prefix)) || codelaw_rules::ENGINE_RULES.iter().any(|r| r.id.starts_with(&prefix))
}

/// The file, its lines checked against the policy and the rule set; `None`
/// when the project has none. A function target outside JS, TS and Python
/// is a load error naming the line (it has no function to find).
pub(crate) fn load(config: &codelaw_config::Config, rules: &codelaw_rules::RuleSet, warnings: &mut Vec<String>) -> Result<Option<IgnoreFile>, EngineError> {
    let policy = policy(config).map_err(|e| config_error(config.root_file(), e))?;
    let Ok(text) = std::fs::read_to_string(config.root_dir.join(IGNORE_FILE)) else { return Ok(None) };
    let mut entries = ignore_file::parse(&text, &policy);
    let mut legacy = Vec::new();
    for e in &mut entries {
        if e.unreadable.is_some() {
            continue;
        }
        if let IgnoreTarget::Function { path, name } = &e.target
            && !Language::from_path(Path::new(path)).is_some_and(codelaw_lang::has_functions)
        {
            return Err(config_error(
                Some(&config.root_dir.join(IGNORE_FILE)),
                format!("line {}: `{path}#{name}` names a function, but `{path}` is not a JavaScript, TypeScript or Python file; name the file (a glob) instead", e.line),
            ));
        }
        if e.legacy {
            legacy.push(e.line.to_string());
            continue;
        }
        for r in &e.rules {
            if r.starts_with("codelaw/") {
                e.incomplete.push(format!("no `{r}`: codelaw's own findings are not exempted"));
            } else if let Some(ns) = r.strip_suffix("/*") {
                if ns.contains(['/', '*']) || !known_namespace(rules, ns) {
                    e.incomplete.push(format!("`{r}` to be a namespace with rules in this project"));
                }
            } else if !known(rules, r) {
                let near = codelaw_config::did_you_mean(r, rules.rules.iter().map(|x| x.rule.id.as_str())).map(|n| format!(" (did you mean `{n}`?)")).unwrap_or_default();
                e.incomplete.push(format!("`{r}` to be a rule this project has{near}"));
            }
        }
    }
    if !legacy.is_empty() {
        warnings.push(format!(
            "{IGNORE_FILE}: line {} names no rule, so it keeps meaning `graph/untested-function` and `graph/function-coverage` until the next minor release; write the rules after the target (`<target> graph/untested-function,graph/function-coverage …`)",
            legacy.join(", ")
        ));
    }
    Ok(Some(IgnoreFile { policy, text, entries }))
}

/// Whether line `e`'s target takes in file `rel` (and, for a function
/// target, a function named `name`; `None` asks about the file only).
pub(crate) fn covers(e: &IgnoreEntry, rel: &str, name: Option<&str>) -> bool {
    match &e.target {
        IgnoreTarget::Function { path, name: n } => path == rel && name.is_none_or(|x| x == n),
        // `.gitignore`-style: a pattern without a `/` matches a name at
        // any depth; a folder covers what is under it.
        IgnoreTarget::Glob(g) => {
            let base = rel.rsplit('/').next().unwrap_or(rel);
            glob(g).is_some_and(|m| m.is_match(rel) || (!g.contains('/') && m.is_match(base))) || rel.starts_with(&format!("{}/", g.trim_end_matches('/')))
        }
    }
}

fn glob(pattern: &str) -> Option<globset::GlobMatcher> {
    globset::GlobBuilder::new(pattern).literal_separator(true).build().ok().map(|g| g.compile_matcher())
}

/// What the lines make of one finding or function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Exempt {
    /// No line covers it.
    No,
    /// A line covers it and holds.
    Holds,
    /// Every line that covers it was revoked by an edit: the findings come
    /// back, as warnings when every such line says `on_touch=warn`.
    Revoked { warn: bool },
}

/// The verdict of the lines that cover something, each with whether an
/// edit touched it under its own `on_touch_scope`.
pub(crate) fn verdict<'a>(covering: impl Iterator<Item = (&'a IgnoreEntry, bool)>, policy: &IgnorePolicy) -> Exempt {
    let mut revoked = Vec::new();
    for (e, touched) in covering {
        match e.on_touch(policy) {
            IgnoreOnTouch::Keep => return Exempt::Holds,
            _ if !touched => return Exempt::Holds,
            t => revoked.push(t),
        }
    }
    if revoked.is_empty() { Exempt::No } else { Exempt::Revoked { warn: revoked.iter().all(|t| *t == IgnoreOnTouch::Warn) } }
}

/// The functions of one file, read once, and how the change set changed
/// each.
#[derive(Default)]
struct Functions {
    of: BTreeMap<String, Vec<Function>>,
    changed: Option<crate::ChangedFunctions>,
}

impl Functions {
    fn of(&mut self, project: &Project, rel: &str, text: Option<&str>) -> &[Function] {
        self.of.entry(rel.to_owned()).or_insert_with(|| {
            let Some(language) = Language::from_path(Path::new(rel)).filter(|l| codelaw_lang::has_functions(*l)) else { return Vec::new() };
            let owned;
            let text = match text {
                Some(t) => t,
                None => {
                    owned = std::fs::read_to_string(project.root().join(rel)).unwrap_or_default();
                    &owned
                }
            };
            codelaw_lang::functions(language, text.strip_prefix('\u{feff}').unwrap_or(text))
        })
    }

    fn change(&mut self, project: &Project, rel: &str, name: &str) -> Change {
        let changed = self.changed.get_or_insert_with(|| project.changed_functions().unwrap_or_default());
        changed.files.iter().find(|f| f.path == rel).and_then(|f| f.functions.iter().find(|c| c.function.name == name)).map(|c| c.change).unwrap_or(Change::Untouched)
    }
}

impl Project {
    /// The lines that cover something on the as-of date: read, complete
    /// and not expired.
    pub(crate) fn live_ignores(&self) -> Vec<&IgnoreEntry> {
        let Some(f) = &self.ignore else { return Vec::new() };
        let today = if f.entries.iter().any(|e| e.expires.is_some()) { self.as_of().date } else { None };
        f.entries
            .iter()
            .filter(|e| e.valid())
            .filter(|e| !matches!(expiry(today, e.expires, None, e.warn_days.unwrap_or(self.exceptions.warn_days)), Expiry::Expired { .. }))
            .collect()
    }

    /// The policy lines are read under (the default without a file).
    pub(crate) fn ignore_policy(&self) -> IgnorePolicy {
        self.ignore.as_ref().map(|f| f.policy).unwrap_or_default()
    }

    /// What the lines make of a finding of `rule` on 1-based `line` of
    /// file `rel`; `text` is the file's, when the caller has it (an
    /// editor's buffer, a fix in progress).
    fn exempt(&self, live: &[&IgnoreEntry], fns: &mut Functions, rule: &str, rel: &str, line: u32, text: Option<&str>) -> Exempt {
        let policy = self.ignore_policy();
        let mut covering: Vec<(&IgnoreEntry, bool)> = Vec::new();
        for e in live.iter().copied().filter(|e| e.names(rule) && covers(e, rel, None)) {
            let touched = match &e.target {
                IgnoreTarget::Glob(_) => self.touched().0.contains(rel),
                IgnoreTarget::Function { name, .. } => {
                    if !fns.of(self, rel, text).iter().any(|f| &f.name == name && f.first_line <= line && line <= f.last_line) {
                        continue;
                    }
                    match e.scope(&policy) {
                        TouchScope::File => self.touched().0.contains(rel),
                        TouchScope::Function => fns.change(self, rel, name) != Change::Untouched,
                    }
                }
            };
            covering.push((e, touched));
        }
        verdict(covering.into_iter(), &policy)
    }

    /// Whether a live line holds a finding of `rule` on `line` of `rel`,
    /// whose text is `text` (`None`: the file on disk): what `codelaw fix` asks before applying a fix
    /// (an exempted finding is not fixed), and what the language server
    /// marks, as it marks a finding the baseline holds (SR-EDITOR
    /// decision 8).
    pub fn codelawignore_holds(&self, rule: &str, rel: &str, line: u32, text: Option<&str>) -> bool {
        if rule.starts_with("codelaw/") || self.ignore.is_none() {
            return false;
        }
        let live = self.live_ignores();
        !live.is_empty() && self.exempt(&live, &mut Functions::default(), rule, rel, line, text) == Exempt::Holds
    }

    /// Drop the findings a live line exempts; a revoked `on_touch=warn`
    /// line brings its findings back as warnings. `codelaw/*` findings are
    /// never exempted, and the test rules of new code are `new_code.rs`'s.
    pub(crate) fn apply_ignore_file(&self, diagnostics: &mut Vec<Diagnostic>) {
        let live = self.live_ignores();
        if live.is_empty() {
            return;
        }
        let mut fns = Functions::default();
        diagnostics.retain_mut(|d| {
            if d.rule.starts_with("codelaw/") || codelaw_rules::TEST_RULES.contains(&d.rule.as_str()) {
                return true;
            }
            match self.exempt(&live, &mut fns, &d.rule, &d.file, d.start.line, None) {
                Exempt::No | Exempt::Revoked { warn: false } => true,
                Exempt::Holds => false,
                Exempt::Revoked { warn: true } => {
                    d.severity = Severity::Warn;
                    true
                }
            }
        });
    }

    /// An engine diagnostic on one line of `.codelawignore`.
    fn ignore_line(&self, line: u32, rule: &str, message: String) -> Option<Diagnostic> {
        self.at_line(IGNORE_FILE, line, rule, message)
    }

    /// `.codelawignore`'s own findings: lines that cannot be read, are
    /// incomplete, expire, or match nothing. `sources`, when the run has
    /// the project's files, saves a walk for the globs.
    pub(crate) fn ignore_file_findings(&self, sources: Option<&[Source]>) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        let Some(file) = &self.ignore else { return out };
        let text = &file.text;
        let today = if file.entries.iter().any(|e| e.expires.is_some()) { self.as_of().date } else { None };
        let index = LineIndex::new(text);
        let mut files: Option<BTreeSet<String>> = sources.map(|s| s.iter().map(|x| x.rel.clone()).collect());
        for e in &file.entries {
            if let Some(bad) = &e.unreadable {
                out.extend(self.ignore_line(e.line, "codelaw/unknown-suppression", format!("This `.codelawignore` line cannot be read, so it exempts nothing: {bad}")));
                continue;
            }
            if !e.incomplete.is_empty() {
                out.extend(self.ignore_line(e.line, INCOMPLETE_RULE, format!("This `.codelawignore` line exempts nothing until it has {}", e.incomplete.join("; "))));
                continue;
            }
            match expiry(today, e.expires, None, e.warn_days.unwrap_or(self.exceptions.warn_days)) {
                Expiry::Expired { expires } => {
                    out.extend(self.ignore_line(e.line, EXPIRED_RULE, format!("`{}` in `.codelawignore` expired on {expires}: it no longer exempts anything", e.target.text())));
                    continue;
                }
                Expiry::Expiring { days, expires } => {
                    out.extend(self.ignore_line(e.line, EXPIRING_RULE, format!("`{}` in `.codelawignore` expires {}", e.target.text(), codelaw_baseline::exception::when(days, expires))));
                }
                Expiry::Holds => {}
            }
            let matches = match &e.target {
                IgnoreTarget::Glob(_) => files.get_or_insert_with(|| self.every_file()).iter().any(|f| covers(e, f, None)),
                IgnoreTarget::Function { path, name } => {
                    let abs = self.root().join(path);
                    match (Language::from_path(&abs).filter(|l| codelaw_lang::has_functions(*l)), std::fs::read_to_string(&abs)) {
                        (Some(l), Ok(t)) => codelaw_lang::functions(l, t.strip_prefix('\u{feff}').unwrap_or(&t)).iter().any(|f| &f.name == name),
                        _ => false,
                    }
                }
            };
            if !matches {
                let what = match &e.target {
                    IgnoreTarget::Glob(g) => format!("`{g}` matches no file"),
                    IgnoreTarget::Function { path, name } => format!("`{path}` has no function `{name}`"),
                };
                if let Some(mut d) = self.ignore_line(e.line, STALE_IGNORE_RULE, format!("{what}: this `.codelawignore` line exempts nothing; `codelaw fix` removes it")) {
                    let start = index.line_start(e.line).unwrap_or(0);
                    let end = index.line_start(e.line + 1).unwrap_or(text.len());
                    d.edits = vec![TextEdit::delete(start, end)];
                    d.fix = FixClass::Auto;
                    out.push(d);
                }
            }
        }
        out
    }

    /// Every file under the root a glob may name, project-relative: the
    /// walk `.gitignore` allows, `.git` left out.
    fn every_file(&self) -> BTreeSet<String> {
        let root = self.root();
        let mut out = BTreeSet::new();
        let walk = ignore::WalkBuilder::new(root).hidden(false).require_git(false).filter_entry(|e| e.file_name() != ".git").build();
        for e in walk.flatten() {
            if e.file_type().is_some_and(|t| t.is_file()) {
                out.insert(crate::discover::rel_path(e.path(), root));
            }
        }
        out
    }

    /// `codelaw fix`: drop the `.codelawignore` lines that match nothing.
    /// Returns how many it removed.
    pub(crate) fn prune_ignore_file(&self, diagnostics: &[Diagnostic]) -> usize {
        let stale: Vec<u32> = diagnostics.iter().filter(|d| d.rule == STALE_IGNORE_RULE && d.file == IGNORE_FILE).map(|d| d.start.line).collect();
        if stale.is_empty() || crate::lint::meta_severity(&[], STALE_IGNORE_RULE).is_none() {
            return 0;
        }
        let path = self.root().join(IGNORE_FILE);
        let Ok(text) = std::fs::read_to_string(&path) else { return 0 };
        if std::fs::write(&path, ignore_file::without_lines(&text, &stale)).is_err() {
            return 0;
        }
        stale.len()
    }

    /// The `.codelawignore` rows of `codelaw exceptions`: one per line and
    /// rule it names (a namespace is one row), `forever` shown as such.
    pub(crate) fn ignore_exceptions(&self) -> Vec<crate::Exception> {
        let Some(file) = &self.ignore else { return Vec::new() };
        let mut out = Vec::new();
        for e in file.entries.iter().filter(|e| e.valid()) {
            let (files, symbol) = match &e.target {
                IgnoreTarget::Glob(g) => (vec![g.clone()], None),
                IgnoreTarget::Function { path, name } => (vec![path.clone()], Some(name.clone())),
            };
            for rule in &e.rules {
                out.push(crate::Exception {
                    kind: crate::ExceptionKind::Ignore,
                    rule: rule.clone(),
                    files: files.clone(),
                    symbol: symbol.clone(),
                    expires: e.expires,
                    forever: e.forever,
                    added_by: e.by.clone(),
                    reason: e.reason.clone(),
                    uses: 1,
                    at: IGNORE_FILE.to_owned(),
                    line: e.line,
                });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(t: &str) -> Vec<IgnoreEntry> {
        ignore_file::parse(t, &IgnorePolicy::default())
    }

    #[test]
    fn verdict_follows_on_touch() {
        let p = IgnorePolicy::default();
        let e = parse("a.py#f js/x on_touch=error expires=2099-01-01: why\nb.py js/x on_touch=warn expires=2099-01-01: why\nc.py js/x on_touch=keep expires=2099-01-01: why\nd.py js/x forever: why");
        assert_eq!(verdict([(&e[0], false)].into_iter(), &p), Exempt::Holds);
        assert_eq!(verdict([(&e[0], true)].into_iter(), &p), Exempt::Revoked { warn: false });
        assert_eq!(verdict([(&e[1], true)].into_iter(), &p), Exempt::Revoked { warn: true });
        assert_eq!(verdict([(&e[0], true), (&e[1], true)].into_iter(), &p), Exempt::Revoked { warn: false });
        assert_eq!(verdict([(&e[2], true)].into_iter(), &p), Exempt::Holds);
        assert_eq!(verdict([(&e[3], true)].into_iter(), &p), Exempt::Holds, "forever is never revoked");
        assert_eq!(verdict(std::iter::empty(), &p), Exempt::No);
    }

    #[test]
    fn targets_cover_like_gitignore() {
        let e = parse("*.py js/x forever: a\nsrc/gen js/x forever: b\nsrc/**/*.ts js/x forever: c\na.py#f js/x forever: d");
        assert!(covers(&e[0], "deep/down/x.py", None));
        assert!(covers(&e[1], "src/gen/a/b.ts", None) && !covers(&e[1], "src/generated.ts", None));
        assert!(covers(&e[2], "src/a/b.ts", None) && !covers(&e[2], "lib/a.ts", None));
        assert!(covers(&e[3], "a.py", None) && covers(&e[3], "a.py", Some("f")) && !covers(&e[3], "a.py", Some("g")));
    }
}
