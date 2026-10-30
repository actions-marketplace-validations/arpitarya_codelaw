//! `deps/licence` (W-129; SR-RULE-PACKS decision 15): the licence of every
//! version a project installs — each direct dependency and every package
//! its lockfile installs through them — against `[deps.licence]`. The
//! licences come from the committed snapshot (`codelaw deps refresh` keeps
//! the registry's per version in use), so the rule reads only files
//! (L5 decision 3). Fix `manual`: replace the package, or a dated ignore.

use super::lock::{all_locked, lockfile_for};
use super::rules::{MANIFESTS, Options, as_of_days, ignored};
use super::snapshot::{Package, SNAPSHOT_PATH, Snapshot};
use super::spdx::{Lists, Verdict, parse};
use super::vulnerable::{lock_name, written_version, yarn_entry_line};
use super::{Ecosystem, folder_of, lock, manifest_deps};
use crate::{Elsewhere, NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, Severity};
use codelaw_lang::SyntaxNode;
use serde_json::Value;
use std::collections::BTreeMap;

/// `[deps.licence]` as the rule reads it.
fn section(options: &Options) -> Options {
    options.get("licence").and_then(Value::as_object).map(|o| o.clone().into_iter().collect()).unwrap_or_default()
}

fn strings(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_owned).collect()
}

fn lists(licence: &Options) -> Lists {
    Lists { allow: strings(licence.get("allow")), deny: strings(licence.get("deny")) }
}

/// `[deps.licence] unknown`: `warn` by default; `off` drops the findings.
fn unknown_severity(licence: &Options) -> Option<Severity> {
    match licence.get("unknown").and_then(Value::as_str) {
        Some("off") => None,
        Some(s) => Severity::parse(s).or(Some(Severity::Warn)),
        None => Some(Severity::Warn),
    }
}

/// Why a licence is not allowed, in the message's words, and its grade.
fn judged(lists: &Lists, licence: Option<&str>) -> Option<(String, &'static str)> {
    let Some(text) = licence else {
        return Some(("names no licence in the registry: find the package's licence, then add it to `[deps.licence] allow` or `deny`, or replace the package".into(), "unknown"));
    };
    match lists.text(text) {
        Verdict::Allowed => None,
        Verdict::Denied => Some((format!("is licensed `{text}`, which `[deps.licence]` denies: replace it, or add a dated `[deps.licence] ignore` entry with a reason"), "denied")),
        Verdict::Unknown if parse(text).is_none() => Some((
            format!("is licensed `{text}`, which is not an SPDX expression: find the package's licence, then add its id to `[deps.licence] allow` or `deny`, or replace the package"),
            "unknown",
        )),
        Verdict::Unknown => Some((format!("is licensed `{text}`, which `[deps.licence]` neither allows nor denies: add it to `allow` or `deny`, or replace the package"), "unknown")),
    }
}

/// The trailing `(denied)` / `(unknown)`.
fn grade_of(message: &str) -> Option<&str> {
    let open = message.rfind(" (")?;
    message[open + 2..].strip_suffix(')')
}

pub struct Licence;

/// The phrase a transitive finding names its lockfile with.
const THROUGH: &str = " (installed through another package, in ";

impl NativeRule for Licence {
    fn id(&self) -> &'static str {
        "deps/licence"
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
    fn severity_of(&self, finding: &NativeFinding, options: &Options) -> Option<Severity> {
        match grade_of(&finding.message) {
            Some("unknown") => unknown_severity(&section(options)),
            _ => None,
        }
    }
    /// A transitive finding from yarn classic's `yarn.lock` has a copy on
    /// the lockfile's entry (SR-LAW-3 decision 10), as `deps/vulnerable`'s.
    fn elsewhere(&self, finding: &NativeFinding, ctx: &NativeContext<'_>) -> Vec<Elsewhere> {
        let Some((_, list)) = finding.message.split_once(" (installed through another package, in yarn.lock) ") else { return Vec::new() };
        let dir = folder_of(ctx.file.path());
        let path = if dir.is_empty() { "yarn.lock".to_owned() } else { format!("{dir}/yarn.lock") };
        let Some(text) = ctx.file.read(&path) else { return Vec::new() };
        // Each named package: `` `name` version (`licence`, grade) ``.
        let list = list.split_once(": ").map(|(_, l)| l).unwrap_or("");
        let mut out = Vec::new();
        for item in list.split(", `").take(LISTED) {
            let mut words = item.trim_start_matches('`').split_whitespace();
            let (Some(name), Some(version)) = (words.next().map(|n| n.trim_end_matches('`')), words.next()) else { continue };
            if let Some(e) = yarn_entry_line(&text, name, version).and_then(|n| Elsewhere::line(path.clone(), &text, n)) {
                out.push(e);
            }
        }
        out
    }
    fn check(&self, node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        let path = ctx.file.path();
        let file = path.rsplit('/').next().unwrap_or(path);
        if !matches!((ctx.language, file), (Language::Json, "package.json") | (Language::Toml, "pyproject.toml")) {
            return;
        }
        let Some(snap) = ctx.file.read(SNAPSHOT_PATH).and_then(|t| Snapshot::parse(&t)) else { return };
        // A snapshot that holds no licence at all predates them: nothing to judge.
        if snap.packages.values().all(|p| p.licence.is_empty()) {
            return;
        }
        let licence = section(ctx.options);
        let lists = lists(&licence);
        let unknown_on = unknown_severity(&licence).is_some();
        let as_of_cell = std::cell::OnceCell::new();
        let as_of = || *as_of_cell.get_or_init(|| as_of_days(ctx));
        let skip = |key: &str| ignored(ctx.options, key, &as_of) || ignored(&licence, key, &as_of);
        let wanted = |j: &Option<(String, &'static str)>| j.as_ref().is_some_and(|(_, g)| *g != "unknown" || unknown_on);
        let dir = folder_of(path);
        let read = |p: &str| ctx.file.read(p);
        let deps = manifest_deps(ctx.language, path, node.get_inner_node(), ctx.source);
        // The direct versions judged here: a lockfile that does not mark
        // them direct (yarn's) lists them again.
        let mut direct: Vec<(String, String)> = Vec::new();
        for eco in [Ecosystem::Npm, Ecosystem::Pypi] {
            let mine: Vec<_> = deps.iter().filter(|d| d.ecosystem == eco).collect();
            if mine.is_empty() {
                continue;
            }
            let specs: BTreeMap<String, String> = mine.iter().map(|d| (lock_name(d), d.spec.clone())).collect();
            let installed = lock::installed(eco, dir, &specs, &read);
            for dep in mine {
                let Some(pkg) = snap.packages.get(&dep.key) else { continue };
                if skip(&dep.key) {
                    continue;
                }
                let Some(version) = installed.get(&lock_name(dep)).cloned().or_else(|| written_version(dep)) else { continue };
                direct.push((lock_name(dep), version.clone()));
                let Some(text) = licence_of(pkg, &version) else { continue };
                let j = judged(&lists, text);
                if !wanted(&j) {
                    continue;
                }
                let (why, grade) = j.unwrap_or_default();
                out.push(NativeFinding { start: dep.start, end: dep.end, message: format!("`{}` {version} {why} ({grade})", dep.name), edits: Vec::new() });
            }
        }
        // The lockfile in this manifest's own folder: its transitive
        // packages' findings sit here, as `deps/vulnerable`'s do.
        let eco = if file == "package.json" { Ecosystem::Npm } else { Ecosystem::Pypi };
        let Some((lockfile, text)) = lockfile_for(eco, dir, &read) else { return };
        if folder_of(&lockfile) != dir {
            return;
        }
        let lock_file = lockfile.rsplit('/').next().unwrap_or(&lockfile).to_owned();
        let line_end = ctx.source.find('\n').unwrap_or(ctx.source.len());
        // One finding for every transitive package: the engine keeps one
        // finding per rule and place, and they all sit on the first line.
        let mut hits: Vec<(String, &'static str)> = Vec::new();
        for l in all_locked(&lock_file, &text).into_iter().filter(|l| !l.direct && !direct.contains(&(l.name.clone(), l.version.clone()))) {
            let key = format!("{}:{}", eco.prefix(), l.name);
            let Some(pkg) = snap.packages.get(&key) else { continue };
            // A transitive package is in the snapshot only with a licence
            // or an advisory: no licence there is not a finding.
            let Some(text) = pkg.licence.get(&l.version) else { continue };
            if skip(&key) {
                continue;
            }
            let j = judged(&lists, Some(text));
            if !wanted(&j) {
                continue;
            }
            let (_, grade) = j.unwrap_or_default();
            hits.push((format!("`{}` {} (`{text}`, {grade})", l.name, l.version), grade));
        }
        if hits.is_empty() {
            return;
        }
        // Denied first; the finding is graded by its worst.
        hits.sort_by_key(|(_, g)| *g != "denied");
        let grade = hits[0].1;
        let n = hits.len();
        let listed: Vec<&str> = hits.iter().take(LISTED).map(|(t, _)| t.as_str()).collect();
        let more = if n > LISTED { format!(" and {} more", n - LISTED) } else { String::new() };
        let what = if n == 1 { "1 package" } else { &format!("{n} packages") };
        out.push(NativeFinding {
            start: 0,
            end: line_end,
            message: format!(
                "{what}{THROUGH}{lock_file}) {} a licence `[deps.licence]` does not allow: {}{more}; replace the dependency that brings {} in, add the licence to `[deps.licence] allow`, or add a dated `[deps.licence] ignore` entry with a reason ({grade})",
                if n == 1 { "has" } else { "have" },
                listed.join(", "),
                if n == 1 { "it" } else { "them" },
            ),
            edits: Vec::new(),
        });
    }
}

/// How many transitive packages a finding names.
const LISTED: usize = 10;

/// The licence the snapshot keeps for `version`: `Some(None)` when the
/// registry named none for a package the refresh asked about, `None` when
/// the snapshot does not know this version (it predates the lockfile).
fn licence_of<'p>(pkg: &'p Package, version: &str) -> Option<Option<&'p str>> {
    match pkg.licence.get(version) {
        Some(t) => Some(Some(t.as_str())),
        None if pkg.licence.is_empty() && !pkg.versions.is_empty() => Some(None),
        None => None,
    }
}
