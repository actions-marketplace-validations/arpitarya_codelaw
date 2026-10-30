//! `deps/runtime-agreement` (W-129; SR-RULE-PACKS decision 15): every
//! place at the project root that names the Node or Python version agrees.
//!
//! - **Node:** `engines.node` (a range) and `volta.node` in `package.json`,
//!   `.nvmrc`, `.node-version`, `.tool-versions` (`nodejs` / `node`),
//!   `mise.toml` / `.mise.toml` (`[tools] node`), each GitHub Actions
//!   workflow's `actions/setup-node` `node-version`, and `Dockerfile`'s
//!   `FROM node:<tag>` (read only).
//! - **Python:** `requires-python` (a range) in `pyproject.toml`,
//!   `.python-version`, `.tool-versions` (`python`), `mise.toml`
//!   (`[tools] python`), `actions/setup-python` `python-version`, and
//!   `FROM python:<tag>`.
//!
//! Every pin must be inside the range and agree with the others — `20`
//! agrees with `20.11.1`, `20.10` does not. `[deps.runtime] source` names
//! the truth per runtime (default `engines.node`, `requires-python`); the
//! pin the others are compared with is the truth when it is a pin, else
//! the first pin, in the order above, that the range allows.
//!
//! A pin in an L3 file (`package.json`, `mise.toml`, a workflow) is
//! reported there, with an edit to the agreed version (`suggest`). A pin in
//! a plain-text file (`.nvmrc`, `.node-version`, `.python-version`,
//! `.tool-versions`, `Dockerfile`) is reported on the root `package.json`
//! (Node) or `pyproject.toml` (Python), with a copy on the text file's line
//! and no edit (SR-LAW-3 decision 10). The rule reads only files.

use super::pep440::{Pep440, requirement};
use super::range::{NpmRange, pin_allowed};
use super::rules::Options;
use crate::data_json::{pairs, string_value, top_object};
use crate::{Elsewhere, NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit, is_text_config};
use codelaw_lang::SyntaxNode;
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Runtime {
    Node,
    Python,
}

impl Runtime {
    fn word(self) -> &'static str {
        match self {
            Runtime::Node => "Node",
            Runtime::Python => "Python",
        }
    }
    /// The source that is the truth when `[deps.runtime] source` does not
    /// say, and the L3 file a text file's finding sits on.
    fn range_source(self) -> &'static str {
        match self {
            Runtime::Node => "engines.node",
            Runtime::Python => "requires-python",
        }
    }
    fn anchor(self) -> &'static str {
        match self {
            Runtime::Node => "package.json",
            Runtime::Python => "pyproject.toml",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Range(String),
    Pin(Vec<u64>),
}

/// One place that names a version.
#[derive(Clone, Debug)]
struct Source {
    /// The `[deps.runtime] source` name (`.nvmrc`, `mise.toml`), or the
    /// workflow's or Dockerfile's path.
    name: &'static str,
    path: String,
    /// The value as written.
    value: String,
    /// Byte offsets of `value` in the file.
    start: usize,
    end: usize,
    kind: Kind,
    /// How the message names it.
    label: String,
    /// A YAML value written without quotes (the edit adds them).
    bare: bool,
}

impl Source {
    fn text_file(&self) -> bool {
        is_text_config(&self.path)
    }
}

fn line_of(text: &str, offset: usize) -> usize {
    text[..offset.min(text.len())].matches('\n').count() + 1
}

/// A pin's release numbers: `v20.11.1`, `20`, `3.12-slim`, `20.x`; `None`
/// for an alias (`lts/*`, `latest`, `system`, `pypy3.10`).
fn pin(text: &str) -> Option<Vec<u64>> {
    let t = text.trim().trim_start_matches('v');
    if !t.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    let core: String = t.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    let parts = core.trim_end_matches('.').split('.').map(|p| p.parse::<u64>().ok()).collect::<Option<Vec<_>>>()?;
    (1..=3).contains(&parts.len()).then_some(parts)
}

fn shown(p: &[u64]) -> String {
    p.iter().map(u64::to_string).collect::<Vec<_>>().join(".")
}

/// Whether two pins agree: equal as far as the shorter goes.
fn agree(a: &[u64], b: &[u64]) -> bool {
    a.iter().zip(b).all(|(x, y)| x == y)
}

/// Whether a range allows a (partial) pin.
fn allowed(rt: Runtime, range: &str, p: &[u64]) -> bool {
    match rt {
        Runtime::Node => NpmRange::parse(range).is_none_or(|r| pin_allowed(&r, p)),
        Runtime::Python => {
            let Some(req) = requirement(&format!("python{range}")) else { return true };
            let ok = |v: &str| Pep440::parse(v).is_some_and(|v| req.clauses.iter().all(|c| c.allows(&v)));
            ok(&shown(p)) || (p.len() < 3 && ok(&format!("{}{}", shown(p), ".999999".repeat(3 - p.len()))))
        }
    }
}

/// A JSON string at `path` in a document's text: its value and offsets.
fn json_string(text: &str, path: &[&str]) -> Option<(String, usize, usize)> {
    let tree = codelaw_lang::parse(Language::Json, text);
    let mut obj = top_object(tree.root().get_inner_node())?;
    for (i, key) in path.iter().enumerate() {
        let (_, _, v) = pairs(obj, text).into_iter().find(|(k, _, _)| k == key)?;
        let v = v?;
        if i + 1 == path.len() {
            if v.kind() != "string" {
                return None;
            }
            let raw = &text[v.start_byte()..v.end_byte()];
            if raw.len() < 2 {
                return None;
            }
            return Some((string_value(v, text), v.start_byte() + 1, v.end_byte() - 1));
        }
        obj = v;
        if obj.kind() != "object" {
            return None;
        }
    }
    None
}

/// The first quoted string after `=` on a line: its value and offsets.
fn quoted_after_eq(line: &str, at: usize) -> Option<(String, usize, usize)> {
    let eq = line.find('=')?;
    let rest = &line[eq + 1..];
    let q = rest.find(['"', '\''])?;
    let quote = rest[q..].chars().next()?;
    let inner = &rest[q + 1..];
    let close = inner.find(quote)?;
    let start = at + eq + 1 + q + 1;
    Some((inner[..close].to_owned(), start, start + close))
}

/// `[tools] node = "20"` (or `nodejs`, `python`) in a mise file.
fn mise(text: &str, keys: &[&str]) -> Option<(String, usize, usize)> {
    let mut in_tools = false;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let t = line.trim();
        if t.starts_with('[') {
            in_tools = t == "[tools]";
        } else if in_tools {
            let key = t.split(['=', ' ']).next().unwrap_or("").trim_matches('"');
            if keys.contains(&key) {
                return quoted_after_eq(line, offset);
            }
        }
        offset += line.len();
    }
    None
}

/// The first non-empty, non-comment line of a version file.
fn version_file(text: &str) -> Option<(String, usize, usize)> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let t = line.trim();
        if !t.is_empty() && !t.starts_with('#') {
            let lead = line.len() - line.trim_start().len();
            return Some((t.to_owned(), offset + lead, offset + lead + t.len()));
        }
        offset += line.len();
    }
    None
}

/// `.tool-versions`: the first version of a tool line.
fn tool_versions(text: &str, tools: &[&str]) -> Option<(String, usize, usize)> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let mut words = line.split_whitespace();
        if words.next().is_some_and(|w| tools.contains(&w))
            && let Some(v) = words.next()
            && !v.starts_with('#')
        {
            let at = line.find(v).map(|i| offset + i)?;
            return Some((v.to_owned(), at, at + v.len()));
        }
        offset += line.len();
    }
    None
}

/// Each `FROM <image>:<tag>` of a Dockerfile whose image is `node` or
/// `python` (any registry or namespace): the tag and its offsets.
fn dockerfile(text: &str, image: &str) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let t = line.trim_start();
        if t.len() > 5 && t[..5].eq_ignore_ascii_case("FROM ") {
            let lead = line.len() - t.len();
            let mut pos = 5;
            for w in t[5..].split(' ') {
                if w.is_empty() || w.starts_with("--") {
                    pos += w.len() + 1;
                    continue;
                }
                let name = w.split('@').next().unwrap_or(w);
                if let Some((img, tag)) = name.rsplit_once(':')
                    && img.rsplit('/').next() == Some(image)
                    && !tag.contains('/')
                {
                    let at = offset + lead + pos + img.len() + 1;
                    out.push((tag.trim_end().to_owned(), at, at + tag.trim_end().len()));
                }
                break;
            }
        }
        offset += line.len();
    }
    out
}

/// Each setup action's version in a workflow: `node-version` of an
/// `actions/setup-node` step (`python-version` of `actions/setup-python`).
/// An expression (`${{ matrix.node }}`) or a list is not read.
fn workflow(text: &str, action: &str, key: &str) -> Vec<(String, usize, usize, bool)> {
    let lines: Vec<(usize, &str)> = {
        let mut o = 0;
        text.split_inclusive('\n')
            .map(|l| {
                let at = o;
                o += l.len();
                (at, l)
            })
            .collect()
    };
    let indent = |l: &str| l.len() - l.trim_start_matches(' ').len();
    let mut out = Vec::new();
    for (i, (at, line)) in lines.iter().enumerate() {
        let t = line.trim_start().trim_start_matches("- ");
        let Some(rest) = t.strip_prefix(key).and_then(|r| r.strip_prefix(':')) else { continue };
        let me = indent(line);
        // The step this key is in: back to the `- ` line above it.
        let Some(step) = (0..i).rev().find(|j| {
            let l = lines[*j].1;
            indent(l) < me && l.trim_start().starts_with("- ")
        }) else {
            continue;
        };
        let dash = indent(lines[step].1);
        let end = (step + 1..lines.len()).find(|j| {
            let l = lines[*j].1;
            !l.trim().is_empty() && !l.trim_start().starts_with('#') && indent(l) <= dash
        });
        let uses_action = lines[step..end.unwrap_or(lines.len())].iter().any(|(_, l)| {
            let u = l.trim_start().trim_start_matches("- ");
            u.strip_prefix("uses:").is_some_and(|u| u.trim().trim_matches(['"', '\'']).starts_with(&format!("{action}@")))
        });
        if !uses_action {
            continue;
        }
        let value = rest.split(" #").next().unwrap_or(rest);
        let v = value.trim();
        if v.is_empty() || v.starts_with("${{") || v.starts_with('[') {
            continue;
        }
        let bare = !(v.starts_with('"') || v.starts_with('\''));
        let inner = v.trim_matches(['"', '\'']);
        let col = line.len() - rest.len() + (rest.len() - rest.trim_start().len()) + if bare { 0 } else { 1 };
        out.push((inner.to_owned(), at + col, at + col + inner.len(), bare));
    }
    out
}

/// Every source of one runtime, in the order the truth is looked for.
fn sources(rt: Runtime, ctx: &NativeContext<'_>) -> Vec<Source> {
    let read = |p: &str| if p == ctx.file.path() { Some(ctx.source.to_owned()) } else { ctx.file.read(p) };
    let mut out = Vec::new();
    let mut push = |name: &'static str, path: &str, _text: &str, (value, start, end): (String, usize, usize), kind: Kind, label: String, bare: bool| {
        out.push(Source { name, path: path.to_owned(), value, start, end, kind, label, bare });
    };
    let pinned = |v: &str| pin(v).map(Kind::Pin);
    match rt {
        Runtime::Node => {
            if let Some(t) = read("package.json") {
                if let Some(v) = json_string(&t, &["engines", "node"]) {
                    let k = Kind::Range(v.0.clone());
                    push("engines.node", "package.json", &t, v, k, "`engines.node` in package.json".into(), false);
                }
                if let Some(v) = json_string(&t, &["volta", "node"])
                    && let Some(k) = pinned(&v.0)
                {
                    push("volta.node", "package.json", &t, v, k, "`volta.node` in package.json".into(), false);
                }
            }
            for (name, file) in [(".nvmrc", ".nvmrc"), (".node-version", ".node-version")] {
                if let Some(t) = read(file)
                    && let Some(v) = version_file(&t)
                    && let Some(k) = pinned(&v.0)
                {
                    let label = format!("`{file}` (line {})", line_of(&t, v.1));
                    push(name, file, &t, v, k, label, false);
                }
            }
        }
        Runtime::Python => {
            if let Some(t) = read("pyproject.toml")
                && let Ok(doc) = toml::from_str::<toml::Table>(&t)
                && let Some(r) = doc.get("project").and_then(|p| p.get("requires-python")).and_then(|r| r.as_str())
            {
                // Its offsets: the string on the `requires-python` line.
                let mut offset = 0;
                for line in t.split_inclusive('\n') {
                    if line.trim_start().starts_with("requires-python")
                        && let Some(v) = quoted_after_eq(line, offset)
                    {
                        push("requires-python", "pyproject.toml", &t, v, Kind::Range(r.to_owned()), "`requires-python` in pyproject.toml".into(), false);
                        break;
                    }
                    offset += line.len();
                }
            }
            if let Some(t) = read(".python-version")
                && let Some(v) = version_file(&t)
                && let Some(k) = pinned(&v.0)
            {
                let label = format!("`.python-version` (line {})", line_of(&t, v.1));
                push(".python-version", ".python-version", &t, v, k, label, false);
            }
        }
    }
    let (tools, mise_keys, image, action, key): (&[&str], &[&str], &str, &str, &str) = match rt {
        Runtime::Node => (&["nodejs", "node"], &["node", "nodejs"], "node", "actions/setup-node", "node-version"),
        Runtime::Python => (&["python"], &["python"], "python", "actions/setup-python", "python-version"),
    };
    if let Some(t) = read(".tool-versions")
        && let Some(v) = tool_versions(&t, tools)
        && let Some(k) = pinned(&v.0)
    {
        let label = format!("`.tool-versions` (line {})", line_of(&t, v.1));
        push(".tool-versions", ".tool-versions", &t, v, k, label, false);
    }
    for file in ["mise.toml", ".mise.toml"] {
        if let Some(t) = read(file)
            && let Some(v) = mise(&t, mise_keys)
            && let Some(k) = pinned(&v.0)
        {
            let label = format!("`[tools] {}` in {file}", mise_keys[0]);
            push("mise.toml", file, &t, v, k, label, false);
        }
    }
    let flows = ctx.file.list(".github/workflows").unwrap_or_default();
    for f in flows.iter().filter(|f| f.ends_with(".yml") || f.ends_with(".yaml")) {
        let p = format!(".github/workflows/{f}");
        let Some(t) = read(&p) else { continue };
        for (value, start, end, bare) in workflow(&t, action, key) {
            if let Some(k) = pinned(&value) {
                let label = format!("`{key}` in {p} (line {})", line_of(&t, start));
                push("workflow", &p, &t, (value, start, end), k, label, bare);
            }
        }
    }
    if let Some(t) = read("Dockerfile") {
        for v in dockerfile(&t, image) {
            if let Some(k) = pinned(&v.0) {
                let label = format!("`FROM {image}:{}` in `Dockerfile` (line {})", v.0, line_of(&t, v.1));
                push("Dockerfile", "Dockerfile", &t, v, k, label, false);
            }
        }
    }
    out
}

/// One disagreement: the source, what is wrong, and the version it should
/// name (`None`: no edit can be offered).
struct Problem<'s> {
    src: &'s Source,
    message: String,
    target: Option<String>,
}

fn problems<'s>(rt: Runtime, srcs: &'s [Source], options: &Options) -> Vec<Problem<'s>> {
    let configured = options.get("runtime").and_then(|r| r.get("source")).and_then(|s| s.get(match rt {
        Runtime::Node => "node",
        Runtime::Python => "python",
    }));
    let truth_name = configured.and_then(Value::as_str).unwrap_or(rt.range_source());
    let range = srcs.iter().find(|s| matches!(s.kind, Kind::Range(_)));
    let range_text = |s: &Source| match &s.kind {
        Kind::Range(r) => r.clone(),
        Kind::Pin(_) => String::new(),
    };
    let truth = srcs.iter().find(|s| s.name == truth_name);
    // The pin every other is compared with.
    let reference = match truth {
        Some(t) if matches!(t.kind, Kind::Pin(_)) => Some(t),
        _ => srcs.iter().find(|s| match (&s.kind, range) {
            (Kind::Pin(p), Some(r)) => allowed(rt, &range_text(r), p),
            (Kind::Pin(_), None) => true,
            _ => false,
        }),
    };
    let ref_pin = reference.and_then(|r| match &r.kind {
        Kind::Pin(p) => Some(p.clone()),
        Kind::Range(_) => None,
    });
    let word = rt.word();
    let mut out = Vec::new();
    for s in srcs {
        if reference.is_some_and(|r| std::ptr::eq(r, s)) {
            // The reference may still be outside the range when it is the truth.
            if let (Kind::Pin(p), Some(r)) = (&s.kind, range)
                && !allowed(rt, &range_text(r), p)
            {
                out.push(Problem {
                    src: r,
                    message: format!("{} `{}` does not allow {word} {}, which {} names and `[deps.runtime] source` makes the truth: widen the range, or change the pin", r.label, range_text(r), s.value, s.label),
                    target: None,
                });
            }
            continue;
        }
        let Kind::Pin(p) = &s.kind else { continue };
        let target = ref_pin.as_ref().map(|rp| {
            let n = p.len().max(1).min(rp.len());
            shown(&rp[..n])
        });
        let to = |t: &Option<String>| match t {
            Some(t) => format!(": set it to {t}"),
            None => format!(": set it to a version {} allows", range.map(|r| r.label.clone()).unwrap_or_default()),
        };
        if let Some(r) = range
            && !allowed(rt, &range_text(r), p)
        {
            out.push(Problem { src: s, message: format!("{word} {} in {} is outside {} `{}`{}", s.value, s.label, r.label, range_text(r), to(&target)), target });
        } else if let (Some(rp), Some(r)) = (&ref_pin, reference)
            && !agree(p, rp)
        {
            out.push(Problem { src: s, message: format!("{word} {} in {} does not agree with {} in {}{}", s.value, s.label, r.value, r.label, to(&target)), target });
        }
    }
    out
}

pub struct RuntimeAgreement;

const LANGUAGES: &[Language] = &[Language::Json, Language::Toml, Language::Yaml];

impl NativeRule for RuntimeAgreement {
    fn id(&self) -> &'static str {
        "deps/runtime-agreement"
    }
    fn languages(&self) -> &'static [Language] {
        LANGUAGES
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["document", "stream"]
    }
    fn config_section(&self) -> Option<&'static str> {
        Some("deps")
    }
    /// A pin in a plain-text file has a copy on its line (SR-LAW-3
    /// decision 10): the first `` `file` (line N) `` the message names.
    fn elsewhere(&self, finding: &NativeFinding, ctx: &NativeContext<'_>) -> Vec<Elsewhere> {
        // Only the subject: the text before what is wrong with it.
        let m = &finding.message;
        let Some(cut) = [" is outside ", " does not agree "].iter().filter_map(|w| m.find(w)).min() else { return Vec::new() };
        let m = &m[..cut];
        let Some(i) = m.find("` (line ") else { return Vec::new() };
        let Some(open) = m[..i].rfind('`') else { return Vec::new() };
        let path = &m[open + 1..i];
        let n: String = m[i + "` (line ".len()..].chars().take_while(char::is_ascii_digit).collect();
        if !is_text_config(path) || path == ctx.file.path() {
            return Vec::new();
        }
        let (Ok(n), Some(text)) = (n.parse::<usize>(), ctx.file.read(path)) else { return Vec::new() };
        Elsewhere::line(path, &text, n).into_iter().collect()
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        // YAML's root is a `stream` of `document`s: once per file.
        if (ctx.language == Language::Yaml) != (node.kind().as_ref() == "stream") {
            return;
        }
        let path = ctx.file.path();
        let runtimes: &[Runtime] = match path {
            "package.json" => &[Runtime::Node],
            "pyproject.toml" => &[Runtime::Python],
            "mise.toml" | ".mise.toml" => &[Runtime::Node, Runtime::Python],
            p if p.starts_with(".github/workflows/") && !p[".github/workflows/".len()..].contains('/') && ctx.language == Language::Yaml => &[Runtime::Node, Runtime::Python],
            _ => return,
        };
        for rt in runtimes {
            let srcs = sources(*rt, ctx);
            if srcs.len() < 2 {
                continue;
            }
            let anchor_range = srcs.iter().find(|s| s.path == path && matches!(s.kind, Kind::Range(_)));
            for p in problems(*rt, &srcs, ctx.options) {
                let s = p.src;
                if s.path == path {
                    let edits = match (&p.target, &s.kind) {
                        // Volta pins an exact version.
                        (Some(t), Kind::Pin(_)) if s.name == "volta.node" && t.split('.').count() < 3 => Vec::new(),
                        (Some(t), Kind::Pin(_)) if s.bare && t.contains('.') => vec![TextEdit::new(s.start, s.end, format!("'{t}'"))],
                        (Some(t), Kind::Pin(_)) => vec![TextEdit::new(s.start, s.end, t.clone())],
                        _ => Vec::new(),
                    };
                    out.push(NativeFinding { start: s.start, end: s.end, message: p.message, edits });
                } else if s.text_file() && path == rt.anchor() {
                    // On the range when this file names one, else its first line.
                    let (start, end) = anchor_range.map(|r| (r.start, r.end)).unwrap_or((0, ctx.source.find('\n').unwrap_or(ctx.source.len())));
                    let message = match &p.target {
                        Some(_) => format!("{} (by hand: codelaw never edits `{}`)", p.message, s.path),
                        None => p.message,
                    };
                    out.push(NativeFinding { start, end, message, edits: Vec::new() });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pins_and_their_files() {
        assert_eq!(pin("v20.11.1"), Some(vec![20, 11, 1]));
        assert_eq!(pin("20-alpine"), Some(vec![20]));
        assert_eq!(pin("3.12-slim"), Some(vec![3, 12]));
        assert_eq!(pin("20.x"), Some(vec![20]));
        assert_eq!(pin("lts/iron"), None);
        assert!(agree(&[20], &[20, 11, 1]) && !agree(&[20, 10], &[20, 11, 1]));
        assert!(allowed(Runtime::Python, ">=3.10,<3.13", &[3, 12]) && !allowed(Runtime::Python, ">=3.10,<3.12", &[3, 12]));
        assert!(allowed(Runtime::Python, ">=3.12.4", &[3, 12]));
        let d = "FROM --platform=linux/amd64 docker.io/library/node:20.11-alpine AS build\nRUN x\nfrom python:3.12-slim\n";
        let got = dockerfile(d, "node");
        assert_eq!(got.len(), 1);
        assert_eq!(&d[got[0].1..got[0].2], "20.11-alpine");
        assert_eq!(&dockerfile(d, "python").iter().map(|(v, a, b)| (v.clone(), d[*a..*b].to_owned())).collect::<Vec<_>>(), &[("3.12-slim".to_owned(), "3.12-slim".to_owned())]);
        let tv = "python 3.12.1 3.11.0\nnodejs 20.11.1\n";
        let (v, a, b) = tool_versions(tv, &["nodejs", "node"]).unwrap();
        assert_eq!((v.as_str(), &tv[a..b]), ("20.11.1", "20.11.1"));
        let m = "[env]\nX = \"1\"\n\n[tools]\nnode = \"20\"\npython = ['3.12', '3.11']\n";
        let (v, a, b) = mise(m, &["python"]).unwrap();
        assert_eq!((v.as_str(), &m[a..b]), ("3.12", "3.12"));
        let wf = "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n      - name: node\n        uses: actions/setup-node@v4\n        with:\n          node-version: 18 # old\n      - uses: actions/setup-node@v4\n        with:\n          node-version: ${{ matrix.node }}\n      - uses: other/thing@v1\n        with:\n          node-version: '16'\n      - uses: actions/setup-python@v5\n        with:\n          python-version: \"3.11\"\n";
        let got = workflow(wf, "actions/setup-node", "node-version");
        assert_eq!(got.iter().map(|(v, a, b, bare)| (v.as_str(), &wf[*a..*b], *bare)).collect::<Vec<_>>(), [("18", "18", true)]);
        let got = workflow(wf, "actions/setup-python", "python-version");
        assert_eq!(got.iter().map(|(v, a, b, bare)| (v.as_str(), &wf[*a..*b], *bare)).collect::<Vec<_>>(), [("3.11", "3.11", false)]);
    }
}
