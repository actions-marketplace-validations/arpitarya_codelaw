//! `@example` code in doc comments through the binary (W-178, SR-LINTER
//! decision 3): with `[lint] jsdoc_examples = true` the example is linted as
//! code of its own and a finding is reported at its place in the comment;
//! without it nothing inside the comment is read as code.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-jsdoc-examples-{name}-{}", std::process::id()));
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

const FILE: &str = "/**\n * Adds.\n * @example\n * var total = add(1, 2);\n */\nexport function add(a, b) {\n  return a + b;\n}\n";

/// A project with the `js` pack installed, `config` after `preview = true`, and `a.js`.
fn project(name: &str, config: &str) -> PathBuf {
    let d = temp(name);
    copy_dir(&repo().join("packs/js"), &d.join("node_modules/@codelaw/rules-js"));
    fs::write(d.join("codelaw.toml"), format!("extends = [\"codelaw:js/markers\"]\npreview = true\nlanguages = [\"javascript\"]\n{config}")).unwrap();
    fs::write(d.join("a.js"), FILE).unwrap();
    d
}

#[test]
fn an_example_is_not_code_until_a_project_asks() {
    let d = project("off", "[rules]\n\"js/no-var\" = \"error\"\n");
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains("js/no-var"), "{out}");
}

#[test]
fn an_example_is_linted_at_its_place_in_the_comment() {
    let d = project("on", "[lint]\njsdoc_examples = true\n[rules]\n\"js/no-var\" = \"error\"\n");
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_ne!(code, 0, "{out}{err}");
    assert!(out.contains("js/no-var"), "{out}");
    assert!(out.contains("a.js:4"), "the finding is on the example's own line: {out}");
}

#[test]
fn a_fix_in_an_example_edits_the_comment_and_leaves_its_margin() {
    let d = project("fix", "[lint]\njsdoc_examples = true\n[rules]\n\"js/curly\" = \"error\"\n");
    fs::write(d.join("a.js"), "/**\n * Adds.\n * @example\n * if (ok) add(1, 2);\n * add(3, 4);\n */\nexport function add(a, b) {\n  return a + b;\n}\n").unwrap();
    let (_, out, err) = codelaw(&d, &["fix", "a.js"]);
    let text = fs::read_to_string(d.join("a.js")).unwrap();
    assert!(text.contains("\n * if (ok) {"), "{out}{err}\n{text}");
    assert!(text.starts_with("/**\n * Adds.\n * @example\n * if (ok) {") && text.contains("\n * add(3, 4);\n */\n"), "{text}");
}

#[test]
fn an_example_that_does_not_parse_is_reported_and_not_linted() {
    let d = project("broken", "[lint]\njsdoc_examples = true\n[rules]\n\"js/no-var\" = \"error\"\n\"js/jsdoc-check-examples\" = \"error\"\n");
    fs::write(d.join("a.js"), "/**\n * @example\n * var total = add(1,\n */\nfunction add() {}\n").unwrap();
    let (code, out, err) = codelaw(&d, &["check", "--format", "agent", "a.js"]);
    assert_ne!(code, 0, "{out}{err}");
    assert!(out.contains("js/jsdoc-check-examples"), "{out}");
    assert!(!out.contains("js/no-var"), "a broken example is not linted: {out}");
}
