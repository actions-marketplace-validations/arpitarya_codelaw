//! Fixes that wait for `codelaw fix` (SR-LAW-4 decision 12, SR-CONFIG
//! decision 11e, SR-RULE-FORMAT decision 2b; W-166), on real projects:
//! `js/no-debugger` ships `on_edit: false`, so the agent hook leaves its
//! `auto` fix and returns it as what is left, while `codelaw fix` applies
//! it; a `[rules]` entry turns it back on, `[fix] on_edit` sets every rule
//! below the entries, and `config --print` and `rules --why` say which line
//! decided.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-w166e-{name}-{}", std::process::id()));
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

const SOURCE: &str = "export function load() {\n  debugger;\n  return 1;\n}\n";

/// A project with the js pack, `config` appended to its `codelaw.toml`.
fn project(name: &str, config: &str) -> PathBuf {
    let d = temp(name);
    let pack = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().join("packs/js");
    copy_dir(&pack, &d.join("node_modules/@codelaw/rules-js"));
    fs::write(d.join("codelaw.toml"), format!("extends = [\"codelaw:js/recommended\"]\npreview = true\n{config}")).unwrap();
    fs::write(d.join("a.js"), SOURCE).unwrap();
    d
}

/// The hook on `a.js`: whether it kept the `debugger`, and its stderr.
fn hook(d: &Path) -> (bool, String) {
    let out = run(d, &["hook", "a.js"]);
    let err = String::from_utf8(out.stderr).unwrap();
    (fs::read_to_string(d.join("a.js")).unwrap().contains("debugger"), err)
}

fn config_print(d: &Path) -> Vec<Value> {
    let out = run(d, &["config", "--print", "--format", "json"]);
    let v: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out.stderr)));
    v.as_array().or(v["values"].as_array()).or(v["config"].as_array()).cloned().unwrap_or_else(|| panic!("{v:#}"))
}

fn entry<'a>(list: &'a [Value], key: &str) -> Option<&'a Value> {
    list.iter().find(|e| e["key"] == key)
}

#[test]
fn the_hook_leaves_an_on_edit_false_fix_and_codelaw_fix_applies_it() {
    let d = project("default", "");
    let (kept, err) = hook(&d);
    assert!(kept, "the hook removed a debugger that waits for `codelaw fix`: {err}");
    assert!(err.contains("js/no-debugger"), "the hook returns it as what is left: {err}");
    let out = run(&d, &["fix", "a.js"]);
    assert!(!fs::read_to_string(d.join("a.js")).unwrap().contains("debugger"), "`codelaw fix` applies it: {}", String::from_utf8_lossy(&out.stderr));
    // `config --print` names the rule's own default; `rules --why` says so.
    let list = config_print(&d);
    let e = entry(&list, "rules.\"js/no-debugger\".on_edit").unwrap_or_else(|| panic!("no on_edit line: {list:#?}"));
    assert_eq!(e["value"], false);
    assert!(e["source"].as_str().unwrap().contains("rule's default"), "{e}");
    let why = String::from_utf8(run(&d, &["rules", "--why", "js/no-debugger"]).stdout).unwrap();
    assert!(why.contains("on edit:    no — its `auto` fix waits for `codelaw fix`"), "{why}");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn a_rule_entry_turns_it_back_on_over_fix_on_edit() {
    let d = project("entry", "\n[fix]\non_edit = false\n\n[rules]\n\"js/no-debugger\" = { on_edit = true }\n");
    let (kept, err) = hook(&d);
    assert!(!kept, "the entry turned on_edit on: {err}");
    let list = config_print(&d);
    assert_eq!(entry(&list, "fix.on_edit").unwrap()["value"], false);
    let e = entry(&list, "rules.\"js/no-debugger\".on_edit").unwrap();
    assert_eq!(e["value"], true);
    assert!(e["source"].as_str().unwrap().ends_with("codelaw.toml"), "{e}");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn fix_on_edit_sets_every_rule_below_the_entries() {
    // `[fix] on_edit = true` is over the rule's own `false`.
    let d = project("fix-true", "\n[fix]\non_edit = true\n");
    let (kept, err) = hook(&d);
    assert!(!kept, "[fix] on_edit = true is over the rule's default: {err}");
    let _ = fs::remove_dir_all(&d);
    // A scope's entry is over `[rules]`.
    let d = project("scope", "\n[rules]\n\"js/no-debugger\" = { on_edit = true }\n\n[scope.all]\nfiles = [\"**\"]\nrules = { \"js/no-debugger\" = { on_edit = false } }\n");
    let (kept, err) = hook(&d);
    assert!(kept, "the scope's entry is over [rules]: {err}");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn on_edit_must_be_true_or_false() {
    let d = project("bad", "\n[fix]\non_edit = \"no\"\n");
    let out = run(&d, &["check", "a.js"]);
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
    let _ = fs::remove_dir_all(&d);
}
