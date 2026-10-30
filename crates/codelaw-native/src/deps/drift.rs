//! `deps/version-drift` (W-129; SR-RULE-PACKS decision 15): across a
//! monorepo's workspaces — npm, yarn and pnpm workspaces, uv workspace
//! members — one dependency declared with ranges that do not agree. Each
//! manifest is checked against the range `[deps.drift] policy` picks:
//! `highest` (the default: the range whose floor is highest) or `root` (the
//! root manifest's, else the highest). Fix `suggest`: the range is
//! rewritten to that one. The rule reads only files (L5 decision 3).

use super::lock::{folders, join, relative};
use super::pep440::{Pep440, normalize, requirement};
use super::range::NpmRange;
use super::rules::{MANIFESTS, Options};
use super::sync::{clause_set, spec_of, spec_span};
use super::{Dep, Ecosystem, folder_of, manifest_deps};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, TextEdit};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::BTreeMap;

/// The groups compared: `peerDependencies` are ranges a package accepts,
/// wide on purpose, so they are not.
const NPM_GROUPS: [&str; 3] = ["dependencies", "devDependencies", "optionalDependencies"];

/// Whether one path segment matches a glob segment (`*`, `?`).
fn seg(p: &[u8], s: &[u8]) -> bool {
    match (p.first(), s.first()) {
        (None, None) => true,
        (Some(b'*'), _) => seg(&p[1..], s) || (!s.is_empty() && seg(p, &s[1..])),
        (Some(b'?'), Some(_)) => seg(&p[1..], &s[1..]),
        (Some(a), Some(b)) if a == b => seg(&p[1..], &s[1..]),
        _ => false,
    }
}

/// Whether a folder matches a workspace glob (`packages/*`, `apps/**`).
pub(super) fn glob_path(pattern: &str, path: &str) -> bool {
    fn walk(p: &[&str], s: &[&str]) -> bool {
        match p.first() {
            None => s.is_empty(),
            Some(&"**") => walk(&p[1..], s) || (!s.is_empty() && walk(p, &s[1..])),
            Some(g) => !s.is_empty() && seg(g.as_bytes(), s[0].as_bytes()) && walk(&p[1..], &s[1..]),
        }
    }
    let pat: Vec<&str> = pattern.trim_start_matches("./").trim_end_matches('/').split('/').filter(|x| !x.is_empty()).collect();
    let segs: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
    walk(&pat, &segs)
}

/// A workspace: its root folder and its member folders (root-relative to
/// the project), the root not among them.
pub(super) struct Workspace {
    pub root: String,
    pub members: Vec<String>,
}

/// The patterns a root declares, `!` exclusions apart.
fn patterns(eco: Ecosystem, folder: &str, read: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    let mut out = Vec::new();
    match eco {
        Ecosystem::Npm => {
            if let Some(doc) = read(&join(folder, "package.json")).and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
                let ws = &doc["workspaces"];
                let list = if ws.is_array() { ws } else { &ws["packages"] };
                out.extend(list.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned));
            }
            if let Some(doc) = read(&join(folder, "pnpm-workspace.yaml")).and_then(|t| serde_saphyr::from_str::<Value>(&t).ok()) {
                out.extend(doc["packages"].as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_owned));
            }
        }
        Ecosystem::Pypi => {
            if let Some(doc) = read(&join(folder, "pyproject.toml")).and_then(|t| toml::from_str::<toml::Table>(&t).ok()) {
                let ws = doc.get("tool").and_then(|t| t.get("uv")).and_then(|u| u.get("workspace"));
                let list = |k: &str| -> Vec<String> { ws.and_then(|w| w.get(k)).and_then(|m| m.as_array()).into_iter().flatten().filter_map(|s| s.as_str().map(str::to_owned)).collect() };
                out.extend(list("members"));
                out.extend(list("exclude").into_iter().map(|e| format!("!{e}")));
            }
        }
    }
    out
}

fn manifest_name(eco: Ecosystem) -> &'static str {
    match eco {
        Ecosystem::Npm => "package.json",
        Ecosystem::Pypi => "pyproject.toml",
    }
}

/// The member folders the patterns name: from the project's file list, or
/// by listing folders for a pattern of plain and `*` segments.
fn members(eco: Ecosystem, root: &str, pats: &[String], ctx: &NativeContext<'_>) -> Vec<String> {
    let (inc, exc): (Vec<&String>, Vec<&String>) = pats.iter().partition(|p| !p.starts_with('!'));
    let keep = |rel: &str| inc.iter().any(|p| glob_path(p, rel)) && !exc.iter().any(|p| glob_path(&p[1..], rel));
    let name = manifest_name(eco);
    let mut out = Vec::new();
    if let Some(files) = ctx.file.project_files() {
        for f in files {
            let Some(d) = f.strip_suffix(name).map(|d| d.trim_end_matches('/')) else { continue };
            if !f.ends_with(&format!("/{name}")) || d == root {
                continue;
            }
            let rel = relative(d, root);
            if (root.is_empty() || d.starts_with(&format!("{root}/"))) && keep(&rel) {
                out.push(d.to_owned());
            }
        }
        return out;
    }
    for p in &inc {
        let mut dirs = vec![root.to_owned()];
        for s in p.trim_start_matches("./").split('/').filter(|s| !s.is_empty()) {
            let mut next = Vec::new();
            for d in &dirs {
                if s.contains(['*', '?']) {
                    if s == "**" {
                        return out;
                    }
                    for n in ctx.file.list(d).into_iter().flatten() {
                        let c = join(d, &n);
                        if seg(s.as_bytes(), n.as_bytes()) && ctx.file.is_dir(&c) {
                            next.push(c);
                        }
                    }
                } else {
                    next.push(join(d, s));
                }
            }
            dirs = next;
        }
        for d in dirs {
            if ctx.file.exists(&join(&d, name)) && keep(&relative(&d, root)) && !out.contains(&d) {
                out.push(d);
            }
        }
    }
    out.sort();
    out
}

/// The workspace the manifest in `dir` belongs to: the nearest folder at
/// or above it whose manifest names workspaces that include it.
pub(super) fn workspace_of(eco: Ecosystem, dir: &str, ctx: &NativeContext<'_>) -> Option<Workspace> {
    let read = |p: &str| ctx.file.read(p);
    for f in folders(dir) {
        let pats = patterns(eco, &f, &read);
        if pats.is_empty() {
            continue;
        }
        let ms = members(eco, &f, &pats, ctx);
        if f == dir || ms.iter().any(|m| m == dir) {
            return Some(Workspace { root: f, members: ms });
        }
        return None;
    }
    None
}

/// One manifest's declared ranges, by snapshot key: npm's range, or a
/// PyPI requirement's specifier. A name declared twice (two markers) is
/// left out, as a range with a protocol is.
fn declared(eco: Ecosystem, text: &str) -> BTreeMap<String, String> {
    let mut out: BTreeMap<String, Option<String>> = BTreeMap::new();
    match eco {
        Ecosystem::Npm => {
            let Ok(doc) = serde_json::from_str::<Value>(text) else { return BTreeMap::new() };
            for g in NPM_GROUPS {
                for (name, v) in doc[g].as_object().into_iter().flatten() {
                    if let Some(s) = v.as_str().filter(|s| NpmRange::parse(s).is_some()) {
                        let k = format!("npm:{name}");
                        let e = out.entry(k).or_insert_with(|| Some(s.to_owned()));
                        if e.as_deref() != Some(s) {
                            *e = None;
                        }
                    }
                }
            }
        }
        Ecosystem::Pypi => {
            let Ok(doc) = toml::from_str::<toml::Table>(text) else { return BTreeMap::new() };
            let strings = |v: Option<&toml::Value>| -> Vec<String> { v.and_then(|v| v.as_array()).into_iter().flatten().filter_map(|s| s.as_str().map(str::to_owned)).collect() };
            let project = doc.get("project");
            let mut reqs = strings(project.and_then(|p| p.get("dependencies")));
            for (_, l) in project.and_then(|p| p.get("optional-dependencies")).and_then(|o| o.as_table()).into_iter().flatten() {
                reqs.extend(strings(Some(l)));
            }
            for (_, l) in doc.get("dependency-groups").and_then(|g| g.as_table()).into_iter().flatten() {
                reqs.extend(strings(Some(l)));
            }
            for r in reqs {
                let Some(req) = requirement(&r) else { continue };
                let k = format!("pypi:{}", req.normalized);
                let s = spec_of(&r);
                match out.get_mut(&k) {
                    Some(e) => {
                        if e.as_deref().map(clause_set) != Some(clause_set(&s)) {
                            *e = None;
                        }
                    }
                    None => {
                        out.insert(k, Some(s));
                    }
                }
            }
        }
    }
    out.into_iter().filter_map(|(k, v)| v.map(|v| (k, v))).collect()
}

/// The lowest version a range allows, to order ranges by.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Floor {
    Npm(semver::Version),
    Pypi(Pep440),
}

fn floor(eco: Ecosystem, spec: &str) -> Option<Floor> {
    match eco {
        Ecosystem::Npm => NpmRange::parse(spec).map(|r| Floor::Npm(r.floor())),
        Ecosystem::Pypi => {
            let req = requirement(&format!("x{spec}"))?;
            let lo = req.clauses.iter().filter(|c| matches!(c.op.as_str(), ">=" | "==" | "===" | "~=" | ">")).filter_map(|c| Pep440::parse(&c.version)).max();
            Some(Floor::Pypi(lo.unwrap_or_else(|| Pep440::parse("0").expect("0 parses"))))
        }
    }
}

/// Whether two ranges say the same: the same text, or for PyPI the same
/// clauses.
fn same(eco: Ecosystem, a: &str, b: &str) -> bool {
    a == b || (eco == Ecosystem::Pypi && clause_set(a).is_some() && clause_set(a) == clause_set(b))
}

pub struct VersionDrift;

impl NativeRule for VersionDrift {
    fn id(&self) -> &'static str {
        "deps/version-drift"
    }
    fn languages(&self) -> &'static [Language] {
        MANIFESTS
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["document"]
    }
    fn config_section(&self) -> Option<&'static str> {
        Some("deps")
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let path = ctx.file.path();
        let file = path.rsplit('/').next().unwrap_or(path);
        let eco = match (ctx.language, file) {
            (Language::Json, "package.json") => Ecosystem::Npm,
            (Language::Toml, "pyproject.toml") => Ecosystem::Pypi,
            _ => return,
        };
        let dir = folder_of(path);
        let Some(ws) = workspace_of(eco, dir, ctx) else { return };
        let drift: Options = ctx.options.get("drift").and_then(Value::as_object).map(|o| o.clone().into_iter().collect()).unwrap_or_default();
        let root_policy = drift.get("policy").and_then(Value::as_str) == Some("root");
        let ignore: Vec<&str> = drift.get("ignore").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).collect();
        // Every manifest of the workspace, the root first.
        let name = manifest_name(eco);
        let mut all: Vec<(String, BTreeMap<String, String>)> = Vec::new();
        for d in std::iter::once(&ws.root).chain(ws.members.iter()) {
            let p = join(d, name);
            let text = if p == path { Some(ctx.source.to_owned()) } else { ctx.file.read(&p) };
            if let Some(t) = text {
                all.push((p, declared(eco, &t)));
            }
        }
        if all.len() < 2 {
            return;
        }
        let mine = declared(eco, ctx.source);
        let root_path = join(&ws.root, name);
        let deps: Vec<Dep> = manifest_deps(ctx.language, path, node.get_inner_node(), ctx.source);
        let mut seen = std::collections::BTreeSet::new();
        for dep in &deps {
            let Some(spec) = mine.get(&dep.key) else { continue };
            if ignore.contains(&dep.key.as_str()) || !seen.insert(dep.key.clone()) {
                continue;
            }
            // Its ranges across the workspace: each distinct one, where.
            let mut ranges: Vec<(String, Vec<String>)> = Vec::new();
            for (p, decl) in &all {
                let Some(s) = decl.get(&dep.key) else { continue };
                match ranges.iter_mut().find(|(r, _)| same(eco, r, s)) {
                    Some((_, at)) => at.push(p.clone()),
                    None => ranges.push((s.clone(), vec![p.clone()])),
                }
            }
            if ranges.len() < 2 {
                continue;
            }
            let at_root = ranges.iter().find(|(_, at)| at.contains(&root_path)).map(|(r, _)| r.clone());
            let highest = ranges
                .iter()
                .filter_map(|(r, at)| floor(eco, r).map(|f| (f, at.len(), at.contains(&root_path), std::cmp::Reverse(r.clone()))))
                .max()
                .map(|(_, _, _, std::cmp::Reverse(r))| r);
            let (target, why) = match (root_policy, at_root) {
                (true, Some(r)) => (r, format!("the root's, `{root_path}` (`[deps.drift] policy = \"root\"`)")),
                _ => match highest {
                    Some(r) => (r, "the highest (`[deps.drift] policy`)".to_owned()),
                    None => continue,
                },
            };
            if same(eco, spec, &target) {
                continue;
            }
            let others: Vec<String> = ranges.iter().filter(|(r, _)| !same(eco, r, spec)).map(|(r, at)| format!("{} in {}", if r.is_empty() { "no range" } else { r }, at.iter().map(|a| format!("`{a}`")).collect::<Vec<_>>().join(", "))).collect();
            let edit = match eco {
                Ecosystem::Npm => TextEdit::new(dep.start, dep.end, target.clone()),
                Ecosystem::Pypi => {
                    let (a, b) = spec_span(&dep.spec);
                    TextEdit::new(dep.start + a, dep.start + b, target.clone())
                }
            };
            let here = if spec.is_empty() { "no range".to_owned() } else { spec.clone() };
            let shown = if eco == Ecosystem::Npm { dep.name.clone() } else { normalize(&dep.name) };
            out.push(NativeFinding {
                start: dep.start,
                end: dep.end,
                message: format!("`{shown}` is {here} here but {} across the workspace: align it to {target}, {why}", others.join("; ")),
                edits: vec![edit],
            });
        }
    }
}
