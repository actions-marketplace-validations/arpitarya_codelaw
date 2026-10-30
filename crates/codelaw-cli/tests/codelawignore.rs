//! `.codelawignore` for every rule (W-158, SR-BASELINE decision 12) on real
//! git repos: a lint rule exempted by glob, by function and by namespace;
//! `on_touch` revoking a line when its file or function is edited; an
//! exempted finding left unfixed by `codelaw fix`; unknown rule ids and
//! `codelaw/` ids incomplete; a stale line; the old spelling and the old
//! table loading with a warning; a function target outside JS, TS and
//! Python refused at load; and the `codelaw exceptions` rows.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const DAY: &str = "2026-10-30";

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-w158-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    fs::canonicalize(d).unwrap()
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap().flatten() {
        let dest = to.join(e.file_name());
        if e.path().is_dir() { copy_dir(&e.path(), &dest) } else { fs::copy(e.path(), dest).map(|_| ()).unwrap() }
    }
}

fn git(cwd: &Path, args: &[&str]) {
    let ok = Command::new("git")
        .args(["-c", "user.email=t@example.com", "-c", "user.name=t", "-c", "commit.gpgsign=false"])
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap()
        .success();
    assert!(ok, "git {args:?}");
}

fn run(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_codelaw"))
        .args(args)
        .current_dir(cwd)
        .env_remove("VIRTUAL_ENV")
        .env_remove("CODELAW_AS_OF")
        .env_remove("CODELAW_CACHE_DIR")
        .env_remove("CODELAW_PROFILE")
        .env_remove("CI")
        .env("NO_COLOR", "1")
        .output()
        .unwrap()
}

fn check(cwd: &Path, args: &[&str]) -> Value {
    let mut all = vec!["check", "--format", "json", "--no-cache", "--as-of", DAY];
    all.extend_from_slice(args);
    let out = run(cwd, &all);
    let text = String::from_utf8(out.stdout).unwrap();
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{e}: {text}\n{}", String::from_utf8_lossy(&out.stderr)))
}

/// `rule severity file:line` for every finding, sorted.
fn found(v: &Value) -> Vec<String> {
    let mut out: Vec<String> = v["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["rule"] != "codelaw/unformatted")
        .map(|d| {
            let line = d["range"]["start"]["line"].as_u64().or(d["line"].as_u64()).unwrap_or(0);
            format!("{} {} {}:{line}", d["rule"].as_str().unwrap(), d["severity"].as_str().unwrap(), d["file"].as_str().unwrap())
        })
        .collect();
    out.sort();
    out
}

const CONFIG: &str = "extends = [\"codelaw:js/recommended\"]\npreview = true\n\n[rules]\n\"js/no-debugger\" = \"error\"\n\"js/no-console\" = \"error\"\n";

/// A committed repo with the js pack: `src/app.ts` has two functions with a
/// `debugger` and a `console.log` each; `scripts/once.js` and
/// `vendor/lib.js` one of each at the top.
fn repo(name: &str) -> PathBuf {
    let d = temp(name);
    git(&d, &["init", "-q", "-b", "main"]);
    let pack = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().join("packs/js");
    copy_dir(&pack, &d.join("node_modules/@codelaw/rules-js"));
    fs::write(d.join(".gitignore"), "node_modules/\n").unwrap();
    fs::write(d.join("codelaw.toml"), CONFIG).unwrap();
    fs::create_dir_all(d.join("src")).unwrap();
    fs::create_dir_all(d.join("scripts")).unwrap();
    fs::create_dir_all(d.join("vendor")).unwrap();
    fs::write(d.join("src/app.ts"), "export function load(): void {\n  debugger;\n  console.log(1);\n}\n\nexport function save(): void {\n  debugger;\n  console.log(2);\n}\n").unwrap();
    fs::write(d.join("scripts/once.js"), "debugger;\nconsole.log(3);\n").unwrap();
    fs::write(d.join("vendor/lib.js"), "debugger;\nconsole.log(4);\n").unwrap();
    git(&d, &["add", "."]);
    git(&d, &["commit", "-q", "-m", "base"]);
    d
}

#[test]
fn a_line_exempts_the_rules_it_names_by_glob_function_and_namespace() {
    let d = repo("exempt");
    let before = found(&check(&d, &[]));
    assert!(before.contains(&"js/no-debugger error src/app.ts:2".to_owned()) && before.contains(&"js/no-console error vendor/lib.js:2".to_owned()), "{before:?}");
    fs::write(
        d.join(".codelawignore"),
        "# exemptions for every rule\n\
scripts/*.js        js/no-console              on_touch=error expires=2027-01-31: one-off scripts print\n\
vendor              js/*                       forever: third-party code, never edited here\n\
src/app.ts#load     js/no-debugger,js/no-console  on_touch=keep expires=2027-01-31 by=a@example.com: until ORD-7\n",
    )
    .unwrap();
    let after = found(&check(&d, &[]));
    assert_eq!(
        after,
        [
            // `save` is not `load`.
            "js/no-console error src/app.ts:8",
            // `scripts/once.js`: only `js/no-console` is named.
            "js/no-debugger error scripts/once.js:1",
            "js/no-debugger error src/app.ts:7",
        ],
        "{after:?}"
    );
    // An exempted finding is not fixed; the rest are.
    run(&d, &["fix", "--no-cache", "--as-of", DAY]);
    let app = fs::read_to_string(d.join("src/app.ts")).unwrap();
    assert_eq!(app.matches("debugger;").count(), 1, "only `save`'s debugger goes:\n{app}");
    assert!(app.split("export function save").next().unwrap().contains("debugger;"), "{app}");
    assert!(fs::read_to_string(d.join("vendor/lib.js")).unwrap().contains("debugger;"), "vendor is exempt from js/*");
    // `codelaw exceptions`: one row per rule a line names, a namespace once.
    let out = run(&d, &["exceptions", "--format", "json", "--as-of", DAY]);
    let ex: Value = serde_json::from_slice(&out.stdout).unwrap();
    let mut rows: Vec<String> = ex["exceptions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] == "codelawignore")
        .map(|r| format!("{} {}{}", r["rule"].as_str().unwrap(), r["files"][0].as_str().unwrap(), r["symbol"].as_str().map(|s| format!("#{s}")).unwrap_or_default()))
        .collect();
    rows.sort();
    assert_eq!(rows, ["js/* vendor", "js/no-console scripts/*.js", "js/no-console src/app.ts#load", "js/no-debugger src/app.ts#load"], "{ex:#}");
}

#[test]
fn an_edit_revokes_a_line_by_file_or_by_function() {
    let d = repo("touch");
    fs::write(
        d.join(".codelawignore"),
        "scripts/once.js   js/no-console  on_touch=error expires=2027-01-31: one-off\n\
src/app.ts#load   js/no-debugger on_touch=warn on_touch_scope=function expires=2027-01-31: until ORD-7\n\
src/app.ts#save   js/no-debugger on_touch=error on_touch_scope=function expires=2027-01-31: until ORD-8\n",
    )
    .unwrap();
    git(&d, &["add", "."]);
    git(&d, &["commit", "-q", "-m", "ignore"]);
    git(&d, &["checkout", "-q", "-b", "feature"]);
    // Edit `scripts/once.js` and only `load` in `src/app.ts`.
    fs::write(d.join("scripts/once.js"), "debugger;\nconsole.log(30);\n").unwrap();
    let app = fs::read_to_string(d.join("src/app.ts")).unwrap();
    fs::write(d.join("src/app.ts"), app.replace("console.log(1)", "console.log(10)")).unwrap();
    let f = found(&check(&d, &["--changed-since", "main"]));
    // The file line is revoked: its finding is back at its own severity.
    assert!(f.contains(&"js/no-console error scripts/once.js:2".to_owned()), "{f:?}");
    // `load` was edited: `on_touch=warn` brings its finding back as a warning.
    assert!(f.contains(&"js/no-debugger warn src/app.ts:2".to_owned()), "{f:?}");
    // `save` was not: function scope keeps its line.
    assert!(!f.iter().any(|x| x.starts_with("js/no-debugger") && x.ends_with("src/app.ts:7")), "{f:?}");
    // Without a change set nothing is touched, and every line holds.
    let f = found(&check(&d, &[]));
    assert!(!f.iter().any(|x| x == "js/no-console error scripts/once.js:2" || x.starts_with("js/no-debugger warn")), "{f:?}");
}

#[test]
fn bad_lines_exempt_nothing_and_stale_lines_are_fixed_away() {
    let d = repo("bad");
    fs::write(
        d.join(".codelawignore"),
        "vendor  js/no-such-rule  forever: typo\n\
vendor  zz/*  forever: no such namespace\n\
vendor  codelaw/unused-suppression  forever: codelaw's own\n\
gone/**  js/no-console  forever: deleted long ago\n\
src/app.ts#nothing  js/no-console  forever: renamed away\n\
vendor  js/no-console  expires=2027-01-31: no on_touch\n",
    )
    .unwrap();
    let v = check(&d, &[]);
    let f = found(&v);
    for line in [1, 2, 3, 6] {
        assert!(f.contains(&format!("codelaw/incomplete-ignore error .codelawignore:{line}")), "line {line}: {f:?}");
    }
    assert!(f.contains(&"codelaw/stale-ignore warn .codelawignore:4".to_owned()) && f.contains(&"codelaw/stale-ignore warn .codelawignore:5".to_owned()), "{f:?}");
    assert!(f.contains(&"js/no-console error vendor/lib.js:2".to_owned()), "an incomplete line exempts nothing: {f:?}");
    let msg = |line: u64| {
        v["diagnostics"].as_array().unwrap().iter().find(|x| x["file"] == ".codelawignore" && x["range"]["start"]["line"].as_u64().or(x["line"].as_u64()) == Some(line)).unwrap()["message"].as_str().unwrap().to_owned()
    };
    assert!(msg(1).contains("`js/no-such-rule` to be a rule this project has"), "{}", msg(1));
    assert!(msg(2).contains("`zz/*` to be a namespace"), "{}", msg(2));
    assert!(msg(3).contains("codelaw's own findings"), "{}", msg(3));
    run(&d, &["fix", "--no-cache", "--as-of", DAY]);
    let after = fs::read_to_string(d.join(".codelawignore")).unwrap();
    assert!(!after.contains("gone/**") && !after.contains("#nothing") && after.contains("js/no-such-rule"), "{after}");
}

#[test]
fn the_old_spellings_load_with_a_warning() {
    let d = repo("old");
    fs::write(d.join("codelaw.toml"), format!("{CONFIG}\n[coverage.ignore]\nrequire_on_touch = false\n")).unwrap();
    // No rule named: the test rules of new code, as before W-158.
    fs::write(d.join(".codelawignore"), "src/app.ts#load  expires=2027-01-31: covered by e2e\n").unwrap();
    let v = check(&d, &[]);
    let warnings: Vec<&str> = v["warnings"].as_array().unwrap().iter().map(|w| w.as_str().unwrap()).collect();
    assert!(warnings.iter().any(|w| w.contains(".codelawignore: line 1 names no rule") && w.contains("graph/untested-function,graph/function-coverage")), "{warnings:?}");
    assert!(warnings.iter().any(|w| w.contains("`[coverage.ignore]` is deprecated") && w.contains("[codelawignore]")), "{warnings:?}");
    let f = found(&v);
    assert!(!f.iter().any(|x| x.contains("incomplete-ignore")), "`require_on_touch = false` is read from the old table: {f:?}");
    assert!(f.contains(&"js/no-debugger error src/app.ts:2".to_owned()), "the old spelling exempts no lint rule: {f:?}");
    // Both tables at once is an error.
    fs::write(d.join("codelaw.toml"), format!("{CONFIG}\n[coverage.ignore]\nrequire_on_touch = false\n\n[codelawignore]\nrequire_on_touch = false\n")).unwrap();
    let out = run(&d, &["check", "--as-of", DAY]);
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
    // The new table alone: no warning.
    fs::write(d.join("codelaw.toml"), format!("{CONFIG}\n[codelawignore]\nrequire_on_touch = false\n")).unwrap();
    fs::write(d.join(".codelawignore"), "src/app.ts#load  js/no-debugger expires=2027-01-31: until ORD-7\n").unwrap();
    let v = check(&d, &[]);
    assert!(v["warnings"].as_array().unwrap().iter().all(|w| !w.as_str().unwrap().contains("codelawignore")), "{v:#}");
    assert!(!found(&v).contains(&"js/no-debugger error src/app.ts:2".to_owned()), "{v:#}");
}

#[test]
fn a_function_target_needs_a_file_with_functions() {
    let d = repo("css");
    fs::write(d.join(".codelawignore"), "src/app.css#main  js/no-console  forever: no functions in CSS\n").unwrap();
    let out = run(&d, &["check", "--as-of", DAY]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("line 1") && err.contains("src/app.css#main") && err.contains("not a JavaScript, TypeScript or Python file"), "{err}");
}
