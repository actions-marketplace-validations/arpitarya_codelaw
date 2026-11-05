//! Target versions, an input to fixes (SR-CONFIG decision 11f;
//! SR-RULE-FORMAT decision 2c; W-166). A rule with `fix_target:` keeps its
//! `auto` findings `auto` only where the file's resolved target meets it;
//! with no target, or one below it, the finding is `suggest` and its note
//! names the line that would make it `auto`.
//!
//! Python comes from the config (a rule entry, a scope, `[targets]`), else
//! from the lower bound of `requires-python` in the nearest `pyproject.toml`
//! above the file. Browsers come from the config, else from a
//! `browserslist` (in `package.json` or `.browserslistrc`) whose every query
//! is `<browser> >= <version>`. Nothing here reaches the network (L5).

use crate::Project;
use crate::discover::FileEntry;
use codelaw_rules::FixTarget;
use std::path::{Path, PathBuf};

/// A resolved target and where it came from; `value` is `None` when the
/// target is unknown, and `source` then says why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolved<T> {
    pub value: Option<T>,
    pub source: String,
}

/// Each browser and its oldest supported version.
pub type Browsers = Vec<(String, Vec<u64>)>;

pub(crate) fn dotted(v: &[u64]) -> String {
    v.iter().map(u64::to_string).collect::<Vec<_>>().join(".")
}

/// `a >= b`, comparing part by part, a missing part as 0.
pub(crate) fn at_least(a: &[u64], b: &[u64]) -> bool {
    let n = a.len().max(b.len());
    let part = |v: &[u64], i: usize| v.get(i).copied().unwrap_or(0);
    for i in 0..n {
        match part(a, i).cmp(&part(b, i)) {
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => {}
        }
    }
    true
}

/// The folders from `abs`'s up to the project root, nearest first.
fn upward<'a>(abs: &'a Path, root: &'a Path) -> impl Iterator<Item = &'a Path> + 'a {
    abs.ancestors().skip(1).take_while(move |d| d.starts_with(root))
}

fn rel(root: &Path, p: &Path) -> String {
    codelaw_core::normalize_path(p.strip_prefix(root).unwrap_or(p))
}

/// `requires-python` in a `pyproject.toml`'s `[project]` table: its value
/// and 1-based line.
fn requires_python(text: &str) -> Option<(String, usize)> {
    let mut in_project = false;
    for (i, line) in text.lines().enumerate() {
        let t = line.trim();
        if t.starts_with('[') {
            in_project = t == "[project]";
            continue;
        }
        if in_project && let Some(rest) = t.strip_prefix("requires-python") {
            let value = rest.trim_start().strip_prefix('=')?.trim();
            let value = value.split('#').next().unwrap_or_default().trim().trim_matches(|c| c == '"' || c == '\'');
            return Some((value.to_owned(), i + 1));
        }
    }
    None
}

/// The queries of a `browserslist`: `package.json`'s key (a string or a
/// list), else `.browserslistrc`'s lines. `Err` names one that is not
/// `<browser> >= <version>`.
fn browserslist(dir: &Path) -> Option<(PathBuf, Result<Browsers, String>)> {
    let parse = |queries: Vec<String>| -> Result<Browsers, String> {
        let qs: Vec<String> = queries.iter().flat_map(|q| q.split(',')).map(|q| q.trim().to_owned()).filter(|q| !q.is_empty()).collect();
        if qs.is_empty() {
            return Err("it lists no query".into());
        }
        qs.iter().map(|q| codelaw_config::parse_browser_target(q).ok_or_else(|| format!("`{q}` is not `<browser> >= <version>`"))).collect()
    };
    let pkg = dir.join("package.json");
    if let Ok(text) = std::fs::read_to_string(&pkg)
        && let Ok(v) = serde_json::from_str::<serde_json::Value>(&text)
        && let Some(b) = v.get("browserslist")
    {
        let got = match b {
            serde_json::Value::String(s) => parse(vec![s.clone()]),
            serde_json::Value::Array(a) => parse(a.iter().filter_map(|q| q.as_str().map(str::to_owned)).collect()),
            _ => Err("its environments are not read; name the browsers in `[targets] browsers`".into()),
        };
        return Some((pkg, got));
    }
    let rc = dir.join(".browserslistrc");
    if let Ok(text) = std::fs::read_to_string(&rc) {
        let lines: Vec<String> = text.lines().map(|l| l.split('#').next().unwrap_or_default().trim().to_owned()).filter(|l| !l.is_empty()).collect();
        let got = if lines.iter().any(|l| l.starts_with('[')) { Err("its environments are not read; name the browsers in `[targets] browsers`".into()) } else { parse(lines) };
        return Some((rc, got));
    }
    None
}

impl Project {
    /// The oldest Python `abs` must run on, for `rule` (SR-CONFIG decision
    /// 11f), and its source.
    pub fn python_target(&self, abs: &Path, rule: Option<&str>) -> Resolved<Vec<u64>> {
        if let Some((v, source)) = self.config.python_target(abs, rule) {
            return Resolved { value: Some(v), source };
        }
        let root = self.root();
        for dir in upward(abs, root) {
            let py = dir.join("pyproject.toml");
            let Ok(text) = std::fs::read_to_string(&py) else { continue };
            let Some((spec, line)) = requires_python(&text) else { continue };
            let at = format!("{}:{line}", rel(root, &py));
            return match codelaw_native::deps::pep440::lower_bound(&spec) {
                Some(v) => Resolved { value: Some(v), source: format!("{at} (`requires-python = \"{spec}\"`)") },
                None => Resolved { value: None, source: format!("unknown: `requires-python = \"{spec}\"` at {at} has no lower bound") },
            };
        }
        Resolved { value: None, source: "unknown: no `[targets] python` and no `requires-python`".into() }
    }

    /// The oldest browsers `abs` must run in, for `rule` (SR-CONFIG
    /// decision 11f), and their source.
    pub fn browsers_target(&self, abs: &Path, rule: Option<&str>) -> Resolved<Browsers> {
        if let Some((v, source)) = self.config.browsers_target(abs, rule) {
            return Resolved { value: Some(v), source };
        }
        let root = self.root();
        for dir in upward(abs, root) {
            let Some((file, got)) = browserslist(dir) else { continue };
            let at = rel(root, &file);
            return match got {
                Ok(v) => Resolved { value: Some(v), source: format!("{at} (browserslist)") },
                Err(why) => Resolved { value: None, source: format!("unknown: the browserslist in {at} is not read: {why}") },
            };
        }
        Resolved { value: None, source: "unknown: no `[targets] browsers` and no browserslist".into() }
    }

    /// Whether `rule`'s `auto` fix may stay `auto` in `entry`: `Err` is
    /// the note of a finding that drops to `suggest`, naming the line that
    /// would make it `auto` (SR-RULE-FORMAT decision 2c).
    pub(crate) fn target_gate(&self, entry: &FileEntry, rule: &str, target: &FixTarget) -> Result<(), String> {
        match target {
            FixTarget::Python(need) => {
                let need_s = dotted(need);
                let got = self.python_target(&entry.abs, Some(rule));
                match got.value {
                    Some(v) if at_least(&v, need) => Ok(()),
                    Some(v) => Err(format!("fix-target: this fix needs Python {need_s}; the target is {} ({})", dotted(&v), got.source)),
                    None => Err(format!("fix-target: this fix needs Python {need_s} and the target is {}; `[targets] python = \"{need_s}\"` in codelaw.toml, or `requires-python = \">={need_s}\"`, makes it `auto`", got.source)),
                }
            }
            FixTarget::Browsers(feature) => {
                let got = self.browsers_target(&entry.abs, Some(rule));
                let Some(browsers) = got.value else {
                    return Err(format!("fix-target: this fix needs the browsers that support `{feature}` unprefixed, and the target is {}; `[targets] browsers = [\"chrome >= …\", …]` in codelaw.toml makes it `auto`", got.source));
                };
                let Some(table) = self.browser_table() else {
                    return Err("fix-target: the browser-support table is not installed; `npm install -D @codelaw/data-browsers` or `pip install codelaw-data-browsers` lets this fix be `auto`".into());
                };
                let Some(row) = table.feature(feature) else {
                    return Err(format!("fix-target: the browser-support table has no row for `{feature}` yet, so this fix stays `suggest`"));
                };
                for (name, min) in &browsers {
                    match row.get(name) {
                        Some(since) if at_least(min, since) => {}
                        Some(since) => return Err(format!("fix-target: `{feature}` is unprefixed from {name} {}; the target is {name} {} ({})", dotted(since), dotted(min), got.source)),
                        None => return Err(format!("fix-target: the table has no {name} version for `{feature}`, so this fix stays `suggest`")),
                    }
                }
                Ok(())
            }
        }
    }

    /// The file's targets, for its cache key: only when a rule with
    /// `fix_target:` could fire (the config's part is in the config
    /// fingerprint; `pyproject.toml` and `browserslist` are not).
    pub(crate) fn targets_key(&self, entry: &FileEntry) -> String {
        let (mut py, mut br) = (false, false);
        for c in self.compiled.iter() {
            match &c.loaded.rule.fix_target {
                Some(FixTarget::Python(_)) if c.applies_to_language(entry.language) => py = true,
                Some(FixTarget::Browsers(_)) if c.applies_to_language(entry.language) => br = true,
                _ => {}
            }
        }
        let mut key = String::new();
        if py {
            key.push_str(&format!("py {:?}", self.python_target(&entry.abs, None)));
        }
        if br {
            key.push_str(&format!("br {:?} {}", self.browsers_target(&entry.abs, None), self.browser_table().map(|t| t.version.as_str()).unwrap_or("none")));
        }
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_part_by_part() {
        assert!(at_least(&[3, 10], &[3, 9]) && at_least(&[3, 9], &[3, 9]) && !at_least(&[3, 8], &[3, 9]));
        assert!(at_least(&[16, 4], &[16]) && !at_least(&[16], &[16, 4]));
    }

    #[test]
    fn requires_python_is_read_from_the_project_table() {
        let text = "[tool.x]\nrequires-python = \">=2\"\n\n[project]\nname = \"x\"\nrequires-python = \">=3.10\"  # floor\n";
        assert_eq!(requires_python(text), Some((">=3.10".into(), 6)));
        assert_eq!(requires_python("[project]\nname = \"x\"\n"), None);
    }
}
