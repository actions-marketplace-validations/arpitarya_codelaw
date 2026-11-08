//! Fixes of project rules that read other files (SR-MULTI-FILE-FIXES,
//! W-162): an undeclared package added to the nearest `package.json` at
//! the version installed, as a choice of `dependencies` or
//! `devDependencies`; a path that names no file pointed at the one nearest
//! in spelling that exists; `styles.x` corrected to the module's nearest
//! class. Each is computed from the project's own files (L5) and offered as
//! `suggest`; one that can't be — nothing installed, no near name — leaves
//! the finding `manual`.

use crate::project::Source;
use codelaw_core::{FileEdit, FixChoice, TextEdit};
use codelaw_graph::{Graph, GraphFinding};
use std::collections::BTreeMap;

/// A project rule's fix: one edit set, or labelled choices.
pub(crate) enum Fix {
    Edits(Vec<TextEdit>),
    Choices(Vec<FixChoice>),
}

fn capture<'a>(f: &'a GraphFinding, key: &str) -> Option<&'a str> {
    f.captures.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

/// The fix of one finding of `query`, when the files give one.
pub(crate) fn fix(rule: &codelaw_graph::GraphRule, f: &GraphFinding, graph: &Graph, sources: &BTreeMap<&str, &Source>, root: &std::path::Path) -> Option<Fix> {
    use codelaw_graph::query::Primitive;
    let wants = |p: Primitive| rule.fix.otherwise.contains(&p) || rule.fix.when_local.contains(&p);
    if wants(Primitive::AddDependency) {
        add_dependency(f, sources, root)
    } else if wants(Primitive::NearestPath) {
        nearest_path(f, graph, sources)
    } else if wants(Primitive::NearestClass) {
        nearest_class(f, graph, sources)
    } else {
        None
    }
}

/// The version of `package` installed for the manifest at `manifest`:
/// `node_modules/<package>/package.json` beside it or above it.
fn installed(package: &str, manifest: &str, sources: &BTreeMap<&str, &Source>, root: &std::path::Path) -> Option<String> {
    let mut dir = manifest.rsplit_once('/').map(|(d, _)| d.to_owned()).unwrap_or_default();
    loop {
        let rel = if dir.is_empty() { format!("node_modules/{package}/package.json") } else { format!("{dir}/node_modules/{package}/package.json") };
        let text = match sources.get(rel.as_str()) {
            Some(s) => Some(s.text.clone()),
            None => std::fs::read_to_string(root.join(&rel)).ok(),
        };
        if let Some(v) = text.and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).and_then(|v| v["version"].as_str().map(str::to_owned)) {
            return Some(v);
        }
        if dir.is_empty() {
            return None;
        }
        dir = dir.rsplit_once('/').map(|(d, _)| d.to_owned()).unwrap_or_default();
    }
}

/// The end of the JSON object or array opening at `open`.
fn close_of(text: &str, open: usize) -> Option<usize> {
    let b = text.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
            }
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// The edit adding `"key": value` as the last member of the object whose
/// `{` is at `open`, in the object's own layout: on its line when it is one
/// line, else on a line of its own at the members' indent.
fn add_member(text: &str, open: usize, key: &str, value: &str) -> Option<TextEdit> {
    let close = close_of(text, open)?;
    let inner = &text[open + 1..close];
    let member = format!("\"{key}\": {value}");
    if inner.trim().is_empty() {
        return Some(TextEdit::new(open + 1, close, format!(" {member} ")));
    }
    let last = open + 1 + inner.trim_end().len();
    if !inner.contains('\n') {
        return Some(TextEdit::insert(last, format!(", {member}")));
    }
    let line_start = text[..last].rfind('\n').map_or(0, |i| i + 1);
    let indent: String = text[line_start..].chars().take_while(|c| *c == ' ' || *c == '\t').collect();
    Some(TextEdit::insert(last, format!(",\n{indent}{member}")))
}

/// `node/no-extraneous-*`: the package added to the nearest manifest, at
/// the version installed, as `dependencies` or `devDependencies` (a test
/// file's import defaults to the second).
fn add_dependency(f: &GraphFinding, sources: &BTreeMap<&str, &Source>, root: &std::path::Path) -> Option<Fix> {
    let package = capture(f, "PACKAGE")?;
    let manifest = capture(f, "MANIFEST")?;
    let text = &sources.get(manifest)?.text;
    let version = installed(package, manifest, sources, root)?;
    let value = format!("\"^{version}\"");
    let top: serde_json::Value = serde_json::from_str(text).ok()?;
    let top_open = text.find('{')?;
    let dev = codelaw_graph::is_test_file(&f.file);
    let mut choices = Vec::new();
    for field in ["dependencies", "devDependencies"] {
        let edit = match top.get(field) {
            Some(serde_json::Value::Object(_)) => {
                let at = text.find(&format!("\"{field}\""))?;
                let open = at + text[at..].find('{')?;
                add_member(text, open, package, &value)?
            }
            Some(_) => return None,
            None => {
                let one_line = !text[top_open..].trim_end().trim_end_matches('}').contains('\n');
                let obj = if one_line { format!("{{ \"{package}\": {value} }}") } else { format!("{{\n    \"{package}\": {value}\n  }}") };
                add_member(text, top_open, field, &obj)?
            }
        };
        choices.push(FixChoice { label: field.to_owned(), default: (field == "devDependencies") == dev, edits: Vec::new(), files: vec![FileEdit::new(manifest, vec![edit])] });
    }
    Some(Fix::Choices(choices))
}

/// The specifier from `from` to the project file `to`, in the form of
/// `like`: without the extension when `like` has none, `index` left out.
fn spec_to(from: &str, to: &str, like: &str) -> String {
    let from_dir: Vec<&str> = from.split('/').collect::<Vec<_>>().split_last().map(|(_, d)| d.to_vec()).unwrap_or_default();
    let to_parts: Vec<&str> = to.split('/').collect();
    let common = from_dir.iter().zip(&to_parts).take_while(|(a, b)| a == b).count();
    let mut parts: Vec<String> = Vec::new();
    for _ in common..from_dir.len() {
        parts.push("..".into());
    }
    parts.extend(to_parts[common..].iter().map(|s| (*s).to_owned()));
    let mut s = parts.join("/");
    let last = like.rsplit('/').next().unwrap_or(like);
    if !last.contains('.') {
        if let Some((stem, _)) = s.rsplit_once('.').filter(|(stem, _)| !stem.ends_with('/') && !stem.is_empty()) {
            s = stem.to_owned();
        }
        if let Some(dir) = s.strip_suffix("/index") {
            s = dir.to_owned();
        }
    }
    if !s.starts_with("..") {
        s = format!("./{s}");
    }
    s
}

/// `node/no-missing-*`: the specifier pointed at the project files nearest
/// in spelling that exist, closest first.
fn nearest_path(f: &GraphFinding, graph: &Graph, sources: &BTreeMap<&str, &Source>) -> Option<Fix> {
    let spec = capture(f, "SOURCE")?;
    let text = &sources.get(f.file.as_str())?.text;
    let (s, e) = f.span;
    let quoted = text.get(s..e)?;
    if quoted.len() < 2 || !quoted.starts_with(['"', '\'', '`']) {
        return None;
    }
    let max = (spec.len() / 4).max(2);
    let mut near: Vec<(usize, String)> = graph
        .modules
        .values()
        .filter(|m| m.path != f.file && m.facts.lang == Some(codelaw_graph::FactLang::Js))
        .map(|m| spec_to(&f.file, &m.path, spec))
        .filter(|c| c != spec)
        .map(|c| (codelaw_core::similar::levenshtein(spec, &c), c))
        .filter(|(d, _)| *d <= max)
        .collect();
    near.sort();
    near.dedup();
    let best = near.first()?.0;
    let near: Vec<String> = near.into_iter().filter(|(d, _)| *d == best).map(|(_, c)| c).take(3).collect();
    let edit = |c: &str| vec![TextEdit::new(s + 1, e - 1, c)];
    if near.len() == 1 {
        return Some(Fix::Edits(edit(&near[0])));
    }
    Some(Fix::Choices(near.iter().enumerate().map(|(i, c)| FixChoice::new(c.as_str(), i == 0, edit(c))).collect()))
}

/// `css-modules/no-undefined-class`: `styles.x` corrected to the module's
/// class nearest in spelling, when one is near and reachable as `styles.y`.
fn nearest_class(f: &GraphFinding, graph: &Graph, sources: &BTreeMap<&str, &Source>) -> Option<Fix> {
    let class = capture(f, "CLASS")?;
    let file = capture(f, "FILE")?;
    let m = graph.modules.get(&f.file)?;
    let text = &sources.get(f.file.as_str())?.text;
    // `styles.x`, or `styles["x"]`: the name itself.
    let (s, e) = match text.get(f.span.0..f.span.1)? {
        t if t == class => f.span,
        t if t.len() == class.len() + 2 && t.starts_with(['"', '\'']) => (f.span.0 + 1, f.span.1 - 1),
        _ => return None,
    };
    let mut names: Vec<String> = Vec::new();
    for imp in &m.facts.imports {
        for t in graph.resolver.resolve_js(&m.path, &imp.source) {
            let codelaw_graph::resolve::Target::File(p) = t else { continue };
            if p.rsplit('/').next() != Some(file) {
                continue;
            }
            if let Some(mf) = graph.modules.get(&p).and_then(|c| c.facts.module.as_ref()) {
                names.extend(mf.classes.iter().map(|c| c.name.clone()));
            }
        }
    }
    let ident = |n: &str| n.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$') && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    let mut near: Vec<(usize, String)> = names.into_iter().filter(|n| ident(n)).map(|n| (codelaw_core::similar::levenshtein(class, &n), n)).filter(|(d, _)| *d <= 2).collect();
    near.sort();
    near.dedup();
    let best = near.first()?.0;
    let near: Vec<String> = near.into_iter().filter(|(d, _)| *d == best).map(|(_, n)| n).collect();
    match near.as_slice() {
        [one] => Some(Fix::Edits(vec![TextEdit::new(s, e, one.as_str())])),
        many => Some(Fix::Choices(many.iter().enumerate().map(|(i, n)| FixChoice::new(n.as_str(), i == 0, vec![TextEdit::new(s, e, n.as_str())])).collect())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_member_keeps_the_object_layout() {
        let one = "{ \"name\": \"a\", \"dependencies\": { \"react\": \"19.0.0\" } }";
        let open = one.find("{ \"react").unwrap();
        let e = add_member(one, open, "lodash", "\"^4.17.21\"").unwrap();
        assert_eq!(codelaw_fix::apply_edits(one, &[e]).unwrap(), "{ \"name\": \"a\", \"dependencies\": { \"react\": \"19.0.0\", \"lodash\": \"^4.17.21\" } }");
        let multi = "{\n  \"dependencies\": {\n    \"react\": \"19.0.0\"\n  }\n}\n";
        let open = multi.find("{\n    ").unwrap();
        let e = add_member(multi, open, "lodash", "\"^4.17.21\"").unwrap();
        assert_eq!(codelaw_fix::apply_edits(multi, &[e]).unwrap(), "{\n  \"dependencies\": {\n    \"react\": \"19.0.0\",\n    \"lodash\": \"^4.17.21\"\n  }\n}\n");
    }

    #[test]
    fn specifiers_keep_the_form_written() {
        assert_eq!(spec_to("src/app.js", "src/helper.js", "./helpers"), "./helper");
        assert_eq!(spec_to("src/a/app.js", "src/helper.ts", "../helpr.js"), "../helper.ts");
        assert_eq!(spec_to("src/app.js", "src/util/index.js", "./utils"), "./util");
    }
}
