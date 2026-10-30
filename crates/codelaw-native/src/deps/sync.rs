//! `deps/lockfile-sync` (W-129; SR-RULE-PACKS decision 15): a manifest and
//! the lockfile that installs it disagree — a dependency in one and not the
//! other, or a range the lockfile did not lock. What each lockfile records
//! of the manifest it was made from:
//!
//! - `package-lock.json`: `packages[""]` (a workspace's `packages["<dir>"]`)
//!   holds each dependency group with the range as written;
//! - `pnpm-lock.yaml`: `importers["."]` (or the workspace's folder), each
//!   dependency's `specifier` (v5: `specifiers`);
//! - `yarn.lock`: one entry per `name@range` descriptor (berry:
//!   `name@npm:range`), so a dependency with no entry is not locked;
//! - `uv.lock`: the project's own `[[package]]` (`source` editable or
//!   virtual), its `[package.metadata] requires-dist` and `requires-dev`;
//! - `poetry.lock`: `[metadata] content-hash`, a SHA-256 of the
//!   pyproject's dependency tables, recomputed here.
//!
//! Fix `manual`: the message names the lock command. A lockfile is never
//! edited. The rule reads only files (L5 decision 3).

use super::lock::{folders, join, relative};
use super::pep440::{Pep440, normalize, requirement};
use super::rules::MANIFESTS;
use super::sha256;
use super::vulnerable::install_command;
use super::{Dep, Ecosystem, folder_of, manifest_deps};
use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// What disagrees: about one dependency line, or about the whole file.
enum Issue {
    Dep(usize, String),
    File(String),
}

const NPM_GROUPS: [&str; 4] = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"];

/// The manifest's dependency names and ranges, every group, from its text
/// (an escaped range the tree reader skips is still a name here).
fn npm_manifest(src: &str, groups: &[&str]) -> BTreeMap<String, String> {
    let Ok(doc) = serde_json::from_str::<Value>(src) else { return BTreeMap::new() };
    let mut out = BTreeMap::new();
    for g in groups {
        for (name, v) in doc[*g].as_object().into_iter().flatten() {
            if let Some(s) = v.as_str() {
                out.entry(name.clone()).or_insert_with(|| s.to_owned());
            }
        }
    }
    out
}

/// The nearest lockfile of `eco` from `dir` up: its path and text.
fn nearest(eco: Ecosystem, dir: &str, read: &dyn Fn(&str) -> Option<String>) -> Option<(String, String)> {
    let names: &[&str] = match eco {
        Ecosystem::Npm => &super::lock::NPM_LOCKFILES,
        Ecosystem::Pypi => &super::lock::PYPI_LOCKFILES,
    };
    for folder in folders(dir) {
        for n in names {
            let p = join(&folder, n);
            if let Some(t) = read(&p) {
                return Some((p, t));
            }
        }
    }
    None
}

/// Compare the manifest's dependencies with what the lockfile recorded of
/// them (`locked`: name → range); `extras` also reports a locked name the
/// manifest no longer has.
fn compare(deps: &[Dep], names: &BTreeMap<String, String>, locked: &BTreeMap<String, Vec<String>>, lockfile: &str, extras: bool, out: &mut Vec<Issue>) {
    for (i, d) in deps.iter().enumerate() {
        if d.ecosystem != Ecosystem::Npm || !names.contains_key(&d.name) {
            continue;
        }
        match locked.get(&d.name) {
            None => out.push(Issue::Dep(i, format!("`{}` {} is not in {lockfile}", d.name, d.spec))),
            Some(s) if !s.contains(&d.spec) => out.push(Issue::Dep(i, format!("`{}` {} — {lockfile} locked it as {}", d.name, d.spec, s.join(" / ")))),
            Some(_) => {}
        }
    }
    if extras {
        for (name, specs) in locked {
            if !names.contains_key(name) {
                out.push(Issue::File(format!("{lockfile} still locks `{name}` {}, which the manifest no longer names", specs.join(" / "))));
            }
        }
    }
}

/// Add a locked range under its name.
fn lock_spec(locked: &mut BTreeMap<String, Vec<String>>, name: &str, spec: &str) {
    let e = locked.entry(name.to_owned()).or_default();
    if !e.iter().any(|s| s == spec) {
        e.push(spec.to_owned());
    }
}

fn package_lock(text: &str, rel: &str, own: bool, deps: &[Dep], src: &str, out: &mut Vec<Issue>) {
    let Ok(doc) = serde_json::from_str::<Value>(text) else { return };
    let names = npm_manifest(src, &NPM_GROUPS);
    let Some(packages) = doc["packages"].as_object() else {
        // lockfileVersion 1: the root's packages by name only.
        if own && rel.is_empty() {
            let locked: BTreeMap<String, Vec<String>> = names.iter().filter(|(n, _)| !doc["dependencies"][n.as_str()].is_null()).map(|(n, s)| (n.clone(), vec![s.clone()])).collect();
            compare(deps, &names, &locked, "package-lock.json", false, out);
        }
        return;
    };
    let Some(entry) = packages.get(rel) else {
        if own {
            out.push(Issue::File("package-lock.json has no entry for this package".into()));
        }
        return;
    };
    let mut locked = BTreeMap::new();
    for g in NPM_GROUPS {
        for (name, v) in entry[g].as_object().into_iter().flatten() {
            if let Some(s) = v.as_str() {
                lock_spec(&mut locked, name, s);
            }
        }
    }
    compare(deps, &names, &locked, "package-lock.json", true, out);
}

/// pnpm records no `peerDependencies` of a project in its importer.
const PNPM_GROUPS: [&str; 3] = ["dependencies", "devDependencies", "optionalDependencies"];

fn pnpm_lock(text: &str, rel: &str, own: bool, deps: &[Dep], src: &str, out: &mut Vec<Issue>) {
    let Ok(doc) = serde_saphyr::from_str::<Value>(text) else { return };
    let importer = if rel.is_empty() { "." } else { rel };
    let names = npm_manifest(src, &PNPM_GROUPS);
    let imp = &doc["importers"][importer];
    let mut locked = BTreeMap::new();
    if imp.is_object() {
        for g in PNPM_GROUPS {
            for (name, e) in imp[g].as_object().into_iter().flatten() {
                if let Some(s) = e["specifier"].as_str() {
                    lock_spec(&mut locked, name, s);
                }
            }
        }
        for (name, s) in imp["specifiers"].as_object().into_iter().flatten() {
            if let Some(s) = s.as_str() {
                lock_spec(&mut locked, name, s);
            }
        }
    } else if importer == "." && doc["importers"].is_null() {
        // v5: the root project's specifiers at the top.
        for (name, s) in doc["specifiers"].as_object().into_iter().flatten() {
            if let Some(s) = s.as_str() {
                lock_spec(&mut locked, name, s);
            }
        }
    } else {
        if own {
            out.push(Issue::File("pnpm-lock.yaml has no importer for this package".into()));
        }
        return;
    }
    compare(deps, &names, &locked, "pnpm-lock.yaml", true, out);
}

fn yarn_lock(text: &str, deps: &[Dep], src: &str, out: &mut Vec<Issue>) {
    let names = npm_manifest(src, &PNPM_GROUPS);
    let mut descriptors = BTreeSet::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with(' ') || line.trim().is_empty() {
            continue;
        }
        for d in line.trim_end_matches(':').split(", ") {
            descriptors.insert(d.trim().trim_matches('"').to_owned());
        }
    }
    for (i, d) in deps.iter().enumerate() {
        // A protocol other than `npm:` (`workspace:`, `file:`, a URL) is
        // written by yarn in its own resolved form: not compared.
        if d.ecosystem != Ecosystem::Npm || !names.contains_key(&d.name) || (d.spec.contains(':') && !d.spec.starts_with("npm:")) {
            continue;
        }
        let plain = format!("{}@{}", d.name, d.spec);
        let berry = format!("{}@npm:{}", d.name, d.spec);
        if !descriptors.contains(&plain) && !descriptors.contains(&berry) {
            out.push(Issue::Dep(i, format!("`{}` {} is not in yarn.lock", d.name, d.spec)));
        }
    }
}

/// A specifier's clauses, comparable: `>=2.31, <3` = `<3,>=2.31.0`.
pub(super) fn clause_set(spec: &str) -> Option<Vec<(String, Pep440, bool)>> {
    let req = requirement(&format!("x{spec}"))?;
    let mut out: Vec<(String, Pep440, bool)> = req.clauses.iter().map(|c| Pep440::parse(&c.version).map(|v| (c.op.clone(), v, c.wildcard))).collect::<Option<_>>()?;
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    Some(out)
}

/// The specifier part of a requirement string (`requests[x]>=2 ; m` → `>=2`).
pub(super) fn spec_of(req: &str) -> String {
    let (a, b) = spec_span(req);
    req[a..b].to_owned()
}

/// The byte range of a requirement's specifier: after the name and
/// extras, before a marker, without spaces or parentheses around it (an
/// empty range where a bare name ends).
pub(super) fn spec_span(req: &str) -> (usize, usize) {
    let body_end = req.find(';').unwrap_or(req.len());
    let lead = req.len() - req.trim_start().len();
    let name_len = req[lead..body_end].find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))).unwrap_or(body_end - lead);
    let mut at = lead + name_len;
    let rest = &req[at..body_end];
    if rest.trim_start().starts_with('[') {
        at += rest.find(']').map(|i| i + 1).unwrap_or(rest.len());
    }
    let tail = &req[at..body_end];
    let start = at + (tail.len() - tail.trim_start_matches([' ', '\t', '(']).len());
    let inner = &req[start..body_end];
    let end = start + inner.trim_end_matches([' ', '\t', ')']).len();
    if end <= start { (at, at) } else { (start, end) }
}

/// One uv `requires-dist` / `requires-dev` entry: its normalised name and
/// specifier.
fn uv_entries(v: Option<&toml::Value>) -> Vec<(String, String)> {
    v.and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter_map(|e| Some((normalize(e.get("name")?.as_str()?), e.get("specifier").and_then(|s| s.as_str()).unwrap_or("").to_owned())))
        .collect()
}

/// Check one manifest requirement against the lock's entries.
fn uv_one(req: &str, locked: &[(String, String)], deps: &[Dep], what: &str, out: &mut Vec<Issue>) {
    let Some(r) = requirement(req) else { return };
    let Some(i) = deps.iter().position(|d| d.ecosystem == Ecosystem::Pypi && d.spec == req) else { return };
    let mine = spec_of(req);
    let same: Vec<&(String, String)> = locked.iter().filter(|(n, _)| *n == r.normalized).collect();
    if same.is_empty() {
        out.push(Issue::Dep(i, format!("`{req}` is not in uv.lock's {what}")));
        return;
    }
    let want = clause_set(&mine);
    if want.is_some() && !same.iter().any(|(_, s)| clause_set(s) == want) {
        let theirs: Vec<&str> = same.iter().map(|(_, s)| if s.is_empty() { "no range" } else { s.as_str() }).collect();
        out.push(Issue::Dep(i, format!("`{req}` — uv.lock locked `{}` as {}", r.name, theirs.join(" / "))));
    }
}

fn uv_lock(text: &str, rel: &str, deps: &[Dep], src: &str, out: &mut Vec<Issue>) {
    let (Ok(lock), Ok(man)) = (toml::from_str::<toml::Table>(text), toml::from_str::<toml::Table>(src)) else { return };
    let Some(project) = man.get("project").and_then(|p| p.as_table()) else { return };
    let Some(name) = project.get("name").and_then(|n| n.as_str()).map(normalize) else { return };
    let local = |p: &toml::Value| {
        p.get("source").and_then(|s| s.as_table()).is_some_and(|s| {
            ["editable", "virtual"].iter().any(|k| s.get(*k).and_then(|v| v.as_str()).is_some_and(|v| v.trim_start_matches("./") == if rel.is_empty() { "." } else { rel } || (rel.is_empty() && v == ".")))
        })
    };
    let packages = lock.get("package").and_then(|p| p.as_array());
    let Some(me) = packages.into_iter().flatten().find(|p| p.get("name").and_then(|n| n.as_str()).map(normalize).as_deref() == Some(name.as_str()) && local(p)) else { return };
    // An older lock has no `metadata`: nothing recorded to compare.
    let Some(meta) = me.get("metadata") else { return };
    let dist = uv_entries(meta.get("requires-dist"));
    let strings = |v: Option<&toml::Value>| -> Vec<String> { v.and_then(|v| v.as_array()).into_iter().flatten().filter_map(|s| s.as_str().map(str::to_owned)).collect() };
    let mut manifest: Vec<String> = strings(project.get("dependencies"));
    for (_, list) in project.get("optional-dependencies").and_then(|o| o.as_table()).into_iter().flatten() {
        manifest.extend(strings(Some(list)));
    }
    for req in &manifest {
        uv_one(req, &dist, deps, "requires-dist", out);
    }
    let names: BTreeSet<String> = manifest.iter().filter_map(|r| requirement(r)).map(|r| r.normalized).collect();
    for (n, s) in &dist {
        if !names.contains(n) {
            out.push(Issue::File(format!("uv.lock still locks `{n}`{} for the project, which the manifest no longer names", if s.is_empty() { String::new() } else { format!(" {s}") })));
        }
    }
    // Dependency groups (and uv's own `dev-dependencies`, its `dev` group).
    let dev = meta.get("requires-dev").and_then(|d| d.as_table());
    let mut groups: Vec<(String, Vec<String>)> = man.get("dependency-groups").and_then(|g| g.as_table()).into_iter().flatten().map(|(g, l)| (g.clone(), strings(Some(l)))).collect();
    if let Some(l) = man.get("tool").and_then(|t| t.get("uv")).and_then(|u| u.get("dev-dependencies")) {
        groups.push(("dev".into(), strings(Some(l))));
    }
    for (g, reqs) in groups {
        let locked = uv_entries(dev.and_then(|d| d.get(&g).or_else(|| d.get(&normalize(&g)))));
        for req in &reqs {
            uv_one(req, &locked, deps, &format!("`{g}` group"), out);
        }
    }
}

/// A value as Python's `json.dumps(..., sort_keys=True)` writes it — what
/// Poetry hashes.
fn py_json(v: &toml::Value, out: &mut String) {
    match v {
        toml::Value::String(s) => py_string(s, out),
        toml::Value::Integer(i) => out.push_str(&i.to_string()),
        toml::Value::Float(f) => out.push_str(&format!("{f:?}")),
        toml::Value::Boolean(b) => out.push_str(if *b { "true" } else { "false" }),
        toml::Value::Datetime(d) => py_string(&d.to_string(), out),
        toml::Value::Array(a) => {
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                py_json(x, out);
            }
            out.push(']');
        }
        toml::Value::Table(t) => py_object(t.iter().map(|(k, v)| (k.as_str(), Some(v))).collect(), out),
    }
}

/// An object with keys sorted; a `None` value is `null`.
fn py_object(mut members: Vec<(&str, Option<&toml::Value>)>, out: &mut String) {
    members.sort_by(|a, b| a.0.cmp(b.0));
    out.push('{');
    for (i, (k, v)) in members.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        py_string(k, out);
        out.push_str(": ");
        match v {
            Some(v) => py_json(v, out),
            None => out.push_str("null"),
        }
    }
    out.push('}');
}

/// A string as `json.dumps` writes it: ASCII only, `\uXXXX` beyond.
fn py_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{u:04x}"));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// The content-hashes the Poetry versions in use would write for this
/// pyproject: 1.x (the `[tool.poetry]` tables at the top, missing legacy
/// keys as `null`), 2.x (with `[project]`'s tables beside them), and 2.x
/// with `[dependency-groups]`.
fn poetry_hashes(man: &toml::Table) -> Vec<String> {
    const LEGACY: [&str; 4] = ["dependencies", "source", "extras", "dev-dependencies"];
    const RELEVANT: [&str; 5] = ["dependencies", "source", "extras", "dev-dependencies", "group"];
    const PROJECT: [&str; 3] = ["requires-python", "dependencies", "optional-dependencies"];
    let empty = toml::Table::new();
    let poetry = man.get("tool").and_then(|t| t.get("poetry")).and_then(|p| p.as_table()).unwrap_or(&empty);
    let project = man.get("project").and_then(|p| p.as_table()).unwrap_or(&empty);
    let hash = |text: String| sha256::hex(text.as_bytes());
    let poetry_members = |with_legacy_nulls: bool| -> Vec<(&str, Option<&toml::Value>)> {
        RELEVANT.iter().filter_map(|k| match poetry.get(*k) {
            Some(v) => Some((*k, Some(v))),
            None if with_legacy_nulls && LEGACY.contains(k) => Some((*k, None)),
            None => None,
        }).collect()
    };
    let mut out = Vec::new();
    // 1.x, and 2.x with no `[project]` tables.
    let mut s = String::new();
    py_object(poetry_members(true), &mut s);
    out.push(hash(s));
    let proj: Vec<(&str, Option<&toml::Value>)> = PROJECT.iter().filter_map(|k| project.get(*k).map(|v| (*k, Some(v)))).collect();
    let groups = man.get("dependency-groups");
    if !proj.is_empty() {
        let mut p = String::new();
        py_object(proj.clone(), &mut p);
        let mut tp = String::new();
        py_object(poetry_members(false), &mut tp);
        let body = format!("\"project\": {p}, \"tool\": {{\"poetry\": {tp}}}");
        out.push(hash(format!("{{{body}}}")));
        if let Some(g) = groups {
            let mut gs = String::new();
            py_json(g, &mut gs);
            out.push(hash(format!("{{\"dependency-groups\": {gs}, {body}}}")));
        }
    } else if let Some(g) = groups {
        let mut members = poetry_members(true);
        members.push(("dependency-groups", Some(g)));
        let mut s = String::new();
        py_object(members, &mut s);
        out.push(hash(s));
    }
    out
}

fn poetry_lock(text: &str, src: &str, out: &mut Vec<Issue>) {
    let (Ok(lock), Ok(man)) = (toml::from_str::<toml::Table>(text), toml::from_str::<toml::Table>(src)) else { return };
    let Some(recorded) = lock.get("metadata").and_then(|m| m.get("content-hash")).and_then(|h| h.as_str()) else { return };
    if !poetry_hashes(&man).iter().any(|h| h == recorded) {
        out.push(Issue::File("poetry.lock's content-hash does not match pyproject.toml's dependencies: the manifest changed after the lock".into()));
    }
}

pub struct LockfileSync;

impl NativeRule for LockfileSync {
    fn id(&self) -> &'static str {
        "deps/lockfile-sync"
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
        let read = |p: &str| ctx.file.read(p);
        let Some((lockpath, text)) = nearest(eco, dir, &read) else { return };
        let lock_dir = folder_of(&lockpath);
        let own = lock_dir == dir;
        let rel = relative(dir, lock_dir);
        let lockfile = lockpath.rsplit('/').next().unwrap_or(&lockpath);
        let deps = manifest_deps(ctx.language, path, node.get_inner_node(), ctx.source);
        let mut issues = Vec::new();
        match lockfile {
            "package-lock.json" => package_lock(&text, &rel, own, &deps, ctx.source, &mut issues),
            "pnpm-lock.yaml" => pnpm_lock(&text, &rel, own, &deps, ctx.source, &mut issues),
            "yarn.lock" => yarn_lock(&text, &deps, ctx.source, &mut issues),
            "uv.lock" => uv_lock(&text, &rel, &deps, ctx.source, &mut issues),
            "poetry.lock" if own => poetry_lock(&text, ctx.source, &mut issues),
            _ => {}
        }
        let cmd = install_command(Some(lockfile), eco);
        let line_end = ctx.source.find('\n').unwrap_or(ctx.source.len());
        for issue in issues {
            let (start, end, what) = match issue {
                Issue::Dep(i, m) => (deps[i].start, deps[i].end, m),
                Issue::File(m) => (0, line_end, m),
            };
            out.push(NativeFinding { start, end, message: format!("{what}: run `{cmd}` so the lockfile follows the manifest (`{lockpath}`)"), edits: Vec::new() });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn python_json_as_poetry_hashes_it() {
        let man: toml::Table = toml::from_str("[tool.poetry.dependencies]\npython = \"^3.10\"\nrequests = { version = \"^2.31\", extras = [\"socks\"] }\n").unwrap();
        let mut s = String::new();
        py_object(vec![("dependencies", man["tool"]["poetry"].get("dependencies")), ("source", None)], &mut s);
        assert_eq!(s, r#"{"dependencies": {"python": "^3.10", "requests": {"extras": ["socks"], "version": "^2.31"}}, "source": null}"#);
        let mut u = String::new();
        py_string("café \u{1F600}", &mut u);
        assert_eq!(u, r#""café 😀""#);
        assert_eq!(spec_of("Requests[socks] >=2.31, <3 ; python_version > '3'"), ">=2.31, <3");
        assert_eq!(clause_set(">=2.31, <3"), clause_set("<3.0,>=2.31.0"));
    }
}
