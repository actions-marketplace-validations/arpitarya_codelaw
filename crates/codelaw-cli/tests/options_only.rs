//! Options-only rules through the binary (W-176, SR-RULE-FORMAT decision
//! 2d): on and unconfigured they say nothing; configured under `[rules]`, in
//! a `[scope.<name>]` or by options alone, they report.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-options-only-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    fs::canonicalize(d).unwrap()
}

fn codelaw(cwd: &Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_codelaw"))
        .args(args)
        .current_dir(cwd)
        .env_remove("VIRTUAL_ENV")
        .env_remove("CODELAW_SITE_PACKAGES")
        .env_remove("CODELAW_AS_OF")
        .env("CODELAW_CACHE_DIR", cwd.join(".cache"))
        .output()
        .unwrap();
    (out.status.code().unwrap_or(-1), String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap())
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            copy_dir(&p, &to.join(e.file_name()));
        } else {
            fs::copy(&p, to.join(e.file_name())).unwrap();
        }
    }
}

/// A project with the `js` pack installed (loaded by a preset that finds nothing in `a.js`), `config` after `preview = true`, and `a.js`.
fn project(name: &str, config: &str) -> PathBuf {
    let d = temp(name);
    copy_dir(&repo().join("packs/js"), &d.join("node_modules/@codelaw/rules-js"));
    fs::write(d.join("codelaw.toml"), format!("extends = [\"codelaw:js/markers\"]\npreview = true\nlanguages = [\"javascript\"]\n{config}")).unwrap();
    fs::write(d.join("a.js"), "const data = load();\nconst copy = data;\n").unwrap();
    d
}

#[test]
fn a_rule_that_is_on_and_unconfigured_says_nothing() {
    let d = project("quiet", "[rules]\n\"js/id-denylist\" = \"error\"\n\"js/id-match\" = \"error\"\n\"js/no-restricted-syntax\" = \"error\"\n");
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains("js/id-"), "{out}");
    assert!(!out.to_lowercase().contains("option"), "no word about a missing option: {out}");
}

#[test]
fn a_rule_reports_once_its_option_is_set() {
    let d = project("set", "[rules]\n\"js/id-denylist\" = { severity = \"error\", options = { names = [\"data\"] } }\n");
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("a.js:1:7 js/id-denylist Identifier 'data' is restricted."), "{out}");
    // `data` read on the second line is not a call's operand, so it is reported too.
    assert!(out.contains("a.js:2:14 js/id-denylist"), "{out}");
}

#[test]
fn a_scope_can_set_the_option_for_some_files_only() {
    let d = project("scope", "[scope.strict]\nfiles = [\"b/**\"]\nrules = { \"js/id-denylist\" = { severity = \"error\", options = { names = [\"data\"] } } }\n");
    fs::create_dir_all(d.join("b")).unwrap();
    fs::write(d.join("b/c.js"), "const data = 1;\n").unwrap();
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js", "b/c.js"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("b/c.js:1:7 js/id-denylist"), "{out}");
    assert!(!out.contains("a.js:"), "{out}");
}

#[test]
fn a_bad_pattern_is_reported_once_and_a_wrong_type_is_a_load_error() {
    let d = project("bad", "[rules]\n\"js/id-match\" = { severity = \"error\", options = { pattern = \"(\" } }\n");
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_eq!(code, 1, "{out}{err}");
    assert_eq!(out.matches("js/id-match").count(), 1, "{out}");
    assert!(out.contains("`pattern` is not a valid regular expression"), "{out}");
    let d = project("wrong", "[rules]\n\"js/id-denylist\" = { severity = \"error\", options = { names = \"data\" } }\n");
    let (code, _, err) = codelaw(&d, &["check", "a.js"]);
    assert_eq!(code, 2, "{err}");
}
