//! Multi-file fixes on real projects (SR-MULTI-FILE-FIXES, W-162): a class
//! renamed in its stylesheet, the page and the script that use it, as one
//! fix; written all or nothing; refused, with every file untouched, when a
//! use would be left dangling; `multi_file = "suggest"` and `"off"` obeyed;
//! never applied by the agent hook; and the fix shown in JSON (by path),
//! SARIF (one artifact change per file) and the agent's text.
//!
//! The veto condition of SR-MULTI-FILE-FIXES points here: a multi-file fix
//! that writes some of its files and not others, or leaves a reference
//! unresolved that resolved before it, fails these tests.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A rule file of the project's own (`.codelaw/rules`) that runs a native
/// rule of the css pack at the rung given.
fn rule(id: &str, native: &str, languages: &str, fix: &str, ext: &str) -> String {
    let reason = if fix == "auto" { "" } else { "fix_reason: \"a rename in several files is a diff to accept\"\n" };
    format!(
        "---\nid: {id}\ntitle: names follow the convention\nlanguages: [{languages}]\nfix: {fix}\n{reason}stability: stable\n---\n\nNames are kebab-case.\n\n**Why:** test.\n\n```codelaw\nnative: {native}\n```\n\n## How to fix\n\n1. Rename it.\n\n## Examples\n\n### {}\n\n#### Don't\n\n```{ext}\n$aB: 1;\n```\n\n#### Do\n\n```{ext}\n$a-b: 1;\n```\n",
        if ext == "scss" { "SCSS" } else { "CSS" }
    )
}

fn project(name: &str, config: &str, files: &[(&str, &str)]) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-multi-file-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    let write = |rel: &str, text: &str| {
        let p = d.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    };
    // A first-party pack's stand-ins: the css pack's native renames at the
    // rung each test needs.
    let pack = "node_modules/@codelaw/rules-css";
    write(&format!("{pack}/codelaw-pack.toml"), "name = \"@codelaw/rules-css\"\nversion = \"0.1.0\"\nmin_codelaw = \"0.1.0\"\nlanguages = [\"css\", \"scss\", \"html\"]\npresets = [\"css/test\"]\n");
    write(&format!("{pack}/rules/css/test-class-name.md"), &rule("css/test-class-name", "css/selector-class-pattern", "css", "suggest", "css"));
    write(&format!("{pack}/rules/css/test-scss-var.md"), &rule("css/test-scss-var", "scss/dollar-variable-pattern", "scss", "auto", "scss"));
    write(
        &format!("{pack}/rules/css/test-bom.md"),
        "---\nid: css/test-bom\ntitle: no byte-order mark\nlanguages: [html]\nfix: auto\nstability: stable\n---\n\nNo mark.\n\n**Why:** test.\n\n```codelaw\nnative: html/no-utf8-bom\n```\n\n## How to fix\n\n1. Remove it.\n\n## Examples\n\n### HTML\n\n#### Don't\n\n```html\n\u{feff}<p></p>\n```\n\n#### Do\n\n```html\n<p></p>\n```\n",
    );
    write(&format!("{pack}/presets/css/test.toml"), "[rules]\n\"css/test-class-name\" = \"warn\"\n\"css/test-scss-var\" = \"warn\"\n\"css/test-bom\" = \"warn\"\n");
    write("codelaw.toml", &format!("extends = [\"codelaw:css/test\"]\n{config}"));
    for (rel, text) in files {
        write(rel, text);
    }
    fs::canonicalize(d).unwrap()
}

fn codelaw(cwd: &Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_codelaw"))
        .args(args)
        .current_dir(cwd)
        .env_remove("VIRTUAL_ENV")
        .env_remove("CODELAW_AS_OF")
        .env_remove("CODELAW_CACHE_DIR")
        .env_remove("CODELAW_PROFILE")
        .env_remove("CI")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    (out.status.code().unwrap_or(-1), String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap())
}

fn read(d: &Path, rel: &str) -> String {
    fs::read_to_string(d.join(rel)).unwrap()
}

const CSS: &str = ".navBar {\n  color: red;\n}\n";
const HTML: &str = "<link rel=\"stylesheet\" href=\"a.css\">\n<nav class=\"navBar main\"></nav>\n";
const JS: &str = "document.querySelector(\".navBar\").classList.add(\"navBar\");\n";

fn rename_project(name: &str, config: &str) -> PathBuf {
    project(name, config, &[("a.css", CSS), ("index.html", HTML), ("app.js", JS)])
}

#[test]
fn a_class_is_renamed_in_every_file_that_uses_it_as_one_fix() {
    let d = rename_project("rename", "");
    // `suggest`: `codelaw fix` leaves it, `--unsafe` applies it.
    codelaw(&d, &["fix", "a.css"]);
    assert_eq!(read(&d, "index.html"), HTML, "a suggestion is not applied without --unsafe");
    let (_, out, err) = codelaw(&d, &["fix", "--unsafe", "a.css"]);
    assert_eq!(read(&d, "a.css"), ".nav-bar {\n  color: red;\n}\n", "{out}{err}");
    assert_eq!(read(&d, "index.html"), HTML.replace("navBar", "nav-bar"));
    assert_eq!(read(&d, "app.js"), JS.replace("navBar", "nav-bar"));
    // A second run changes nothing, and the rule no longer fires.
    let (_, out, _) = codelaw(&d, &["check", "a.css"]);
    assert!(!out.contains("css/test-class-name"), "{out}");
    let leftovers: Vec<String> = fs::read_dir(&d).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| n.ends_with(".tmp")).collect();
    assert!(leftovers.is_empty(), "{leftovers:?}");
}

#[test]
fn every_output_shows_the_files_a_fix_edits() {
    let d = rename_project("outputs", "");
    let (_, out, _) = codelaw(&d, &["check", "a.css", "--format", "json"]);
    let v: Value = serde_json::from_str(&out).unwrap();
    let f = &v["diagnostics"].as_array().unwrap().iter().find(|x| x["rule"] == "css/test-class-name").unwrap()["fix"];
    assert_eq!(f["class"], "suggest");
    let paths: Vec<&str> = f["files"].as_array().unwrap().iter().map(|x| x["path"].as_str().unwrap()).collect();
    assert_eq!(paths, ["app.js", "index.html"], "{f}");
    assert_eq!(f["files"][0]["edits"].as_array().unwrap().len(), 2, "{f}");

    let (_, out, _) = codelaw(&d, &["check", "a.css", "--format", "sarif"]);
    let v: Value = serde_json::from_str(&out).unwrap();
    let r = v["runs"][0]["results"].as_array().unwrap().iter().find(|x| x["ruleId"] == "css/test-class-name").unwrap();
    let uris: Vec<&str> = r["fixes"][0]["artifactChanges"].as_array().unwrap().iter().map(|c| c["artifactLocation"]["uri"].as_str().unwrap()).collect();
    assert_eq!(uris.len(), 3, "{r}");
    assert!(uris.iter().any(|u| u.ends_with("index.html")) && uris.iter().any(|u| u.ends_with("app.js")), "{uris:?}");

    let (_, out, _) = codelaw(&d, &["check", "a.css", "--format", "agent"]);
    assert!(out.contains("edits 3 files"), "{out}");
    let (_, out, _) = codelaw(&d, &["check", "a.css"]);
    assert!(out.contains("suggest, edits 3 files"), "{out}");
}

#[test]
fn a_use_left_dangling_refuses_the_fix_and_no_file_changes() {
    // `navBar` is a word of the page too: the graph counts it as a possible
    // use of the class, which the rename does not reach.
    let html = "<nav class=\"navBar\">the navBar</nav>\n";
    let d = project("dangling", "", &[("a.css", CSS), ("index.html", html)]);
    let (_, out, _) = codelaw(&d, &["fix", "--unsafe", "--format", "json", "a.css"]);
    assert_eq!(read(&d, "a.css"), CSS);
    assert_eq!(read(&d, "index.html"), html);
    let v: Value = serde_json::from_str(&out).unwrap();
    let diags = v["diagnostics"].as_array().or(v["remaining"].as_array()).cloned().unwrap_or_default();
    let d0 = diags.iter().find(|x| x["rule"] == "css/test-class-name").unwrap_or_else(|| panic!("{out}"));
    assert_eq!(d0["fix"]["class"], "manual", "{d0}");
    assert!(d0["fix"]["note"].as_str().unwrap().contains("dangling"), "{d0}");
}

#[test]
fn multi_file_off_and_suggest_are_obeyed() {
    let d = rename_project("off", "[fix]\nmulti_file = \"off\"\n");
    let (_, out, _) = codelaw(&d, &["check", "a.css", "--format", "json"]);
    let v: Value = serde_json::from_str(&out).unwrap();
    let f = &v["diagnostics"].as_array().unwrap().iter().find(|x| x["rule"] == "css/test-class-name").unwrap()["fix"];
    assert_eq!(f["class"], "manual");
    assert_eq!(f["exception"], "restructure");
    assert!(f["steps"][0].as_str().unwrap().contains("index.html"), "{f}");
    codelaw(&d, &["fix", "--unsafe", "a.css"]);
    assert_eq!(read(&d, "index.html"), HTML);
    let (_, out, _) = codelaw(&d, &["rules", "--why", "css/test-class-name"]);
    assert!(out.contains("multi-file: off"), "{out}");

    // Per rule, `suggest` keeps an `auto` rename a diff.
    let scss = [("_vars.scss", "$mainColor: red;\n"), ("app.scss", "@use \"vars\";\n.a {\n  color: vars.$mainColor;\n}\n")];
    let d = project("suggest", "", &scss);
    let (_, out, _) = codelaw(&d, &["check", "_vars.scss", "--format", "json"]);
    let v: Value = serde_json::from_str(&out).unwrap();
    let class = |v: &Value| v["diagnostics"].as_array().unwrap().iter().find(|x| x["rule"] == "css/test-scss-var").unwrap()["fix"]["class"].clone();
    assert_eq!(class(&v), "auto", "every user reaches it through `@use`: {out}");
    let d = project("suggest2", "[rules.\"css/test-scss-var\"]\nseverity = \"warn\"\nmulti_file = \"suggest\"\n", &scss);
    let (_, out, _) = codelaw(&d, &["check", "_vars.scss", "--format", "json"]);
    assert_eq!(class(&serde_json::from_str(&out).unwrap()), "suggest", "{out}");
}

#[test]
fn the_agent_hook_never_applies_a_multi_file_fix_and_codelaw_fix_does() {
    let scss = [("_vars.scss", "$mainColor: red;\n"), ("app.scss", "@use \"vars\";\n.a {\n  color: vars.$mainColor;\n}\n")];
    let d = project("hook", "", &scss);
    let (_, _, err) = codelaw(&d, &["hook", "_vars.scss"]);
    assert_eq!(read(&d, "_vars.scss"), scss[0].1, "the hook wrote a multi-file fix: {err}");
    assert_eq!(read(&d, "app.scss"), scss[1].1);
    assert!(err.contains("edits 2 files, never applied by this hook"), "{err}");
    codelaw(&d, &["fix", "_vars.scss"]);
    assert_eq!(read(&d, "_vars.scss"), "$main-color: red;\n");
    assert_eq!(read(&d, "app.scss"), "@use \"vars\";\n.a {\n  color: vars.$main-color;\n}\n");
}

#[test]
fn a_dry_run_writes_nothing_and_reports_what_it_would_fix() {
    let d = rename_project("dry", "");
    let (_, out, err) = codelaw(&d, &["fix", "--unsafe", "--dry-run", "a.css"]);
    assert_eq!(read(&d, "a.css"), CSS);
    assert_eq!(read(&d, "index.html"), HTML);
    assert_eq!(read(&d, "app.js"), JS);
    let said = format!("{out}{err}");
    for f in ["a.css", "app.js", "index.html"] {
        assert!(said.contains(&format!("would fix {f} (css/test-class-name ×1)")), "{said}");
    }
    assert!(said.contains("0 problems"), "what is left is what the fix would leave: {said}");
}

#[test]
fn a_byte_order_mark_is_removed_by_codelaw_fix_and_not_by_the_hook() {
    let d = project("bom", "", &[("page.html", "\u{feff}<p>x</p>\n")]);
    let (_, _, err) = codelaw(&d, &["hook", "page.html"]);
    assert!(fs::read(d.join("page.html")).unwrap().starts_with("\u{feff}".as_bytes()), "the hook removed the mark: {err}");
    let (_, out, err) = codelaw(&d, &["fix", "page.html"]);
    assert_eq!(fs::read(d.join("page.html")).unwrap(), b"<p>x</p>\n", "{out}{err}");
    let (_, out, _) = codelaw(&d, &["check", "page.html"]);
    assert!(!out.contains("css/test-bom"), "{out}");
}
