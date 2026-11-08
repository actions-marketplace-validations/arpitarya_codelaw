//! Target versions as an input to fixes (SR-CONFIG decision 11f,
//! SR-RULE-FORMAT decision 2c; W-166), on real projects:
//! `py/lru-cache-with-maxsize-none` (`fix_target: { python: "3.9" }`) is
//! `auto` only where the file's Python target is 3.9 or later — from
//! `requires-python`'s lower bound, `[targets] python`, a scope's or a rule
//! entry's `targets.python` — and `suggest` elsewhere, its note naming the
//! line that would make it `auto`; `config --print` and `rules --why` show
//! the target and its source; a bad target is exit 2.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const RULE: &str = "py/lru-cache-with-maxsize-none";
const SOURCE: &str = "import functools\n\n\n@functools.lru_cache(maxsize=None)\ndef f():\n    return 1\n";

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-w166t-{name}-{}", std::process::id()));
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
        .env_remove("CODELAW_SITE_PACKAGES")
        .env_remove("CODELAW_AS_OF")
        .env_remove("CODELAW_CACHE_DIR")
        .env_remove("CODELAW_PROFILE")
        .env_remove("CI")
        .env("NO_COLOR", "1")
        .output()
        .unwrap()
}

/// A project with the python pack (PyPI only), the rule on, `config`
/// appended to its `codelaw.toml` and, when given, a `pyproject.toml`.
fn project(name: &str, config: &str, requires_python: Option<&str>) -> PathBuf {
    let d = temp(name);
    let pack = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().join("packs/python");
    copy_dir(&pack, &d.join(".venv/lib/python3.12/site-packages/codelaw_rules_python"));
    fs::write(d.join("codelaw.toml"), format!("extends = [\"codelaw:py/recommended\"]\npreview = true\n\n[rules]\n\"{RULE}\" = \"warn\"\n{config}")).unwrap();
    if let Some(r) = requires_python {
        fs::write(d.join("pyproject.toml"), format!("[project]\nname = \"x\"\nrequires-python = \"{r}\"\n")).unwrap();
    }
    fs::create_dir_all(d.join("legacy")).unwrap();
    fs::write(d.join("app.py"), SOURCE).unwrap();
    fs::write(d.join("legacy/old.py"), SOURCE).unwrap();
    d
}

/// The rule's finding in `file`: its fix class and note.
fn finding(d: &Path, file: &str) -> (String, String) {
    let out = run(d, &["check", file, "--format", "json", "--no-cache"]);
    let v: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out.stderr)));
    let d = v["diagnostics"].as_array().unwrap().iter().find(|d| d["rule"] == RULE).unwrap_or_else(|| panic!("no {RULE}: {v:#}"));
    (d["fix"]["class"].as_str().unwrap_or_default().to_owned(), d["fix"]["note"].as_str().unwrap_or_default().to_owned())
}

fn printed(d: &Path, key: &str) -> Value {
    let out = run(d, &["config", "--print", "--format", "json"]);
    let v: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out.stderr)));
    let list = v.as_array().or(v["values"].as_array()).or(v["config"].as_array()).cloned().unwrap();
    list.into_iter().find(|e| e["key"] == key).unwrap_or_else(|| panic!("no {key}"))
}

#[test]
fn with_no_target_the_fix_is_suggest_and_says_which_line_makes_it_auto() {
    let d = project("none", "", None);
    let (fix, note) = finding(&d, "app.py");
    assert_eq!(fix, "suggest", "{note}");
    assert!(note.contains("`[targets] python = \"3.9\"`") && note.contains("requires-python"), "{note}");
    // `codelaw fix` leaves a `suggest` finding.
    run(&d, &["fix", "app.py"]);
    assert!(fs::read_to_string(d.join("app.py")).unwrap().contains("lru_cache(maxsize=None)"));
    let e = printed(&d, "targets.python");
    assert_eq!(e["value"], Value::Null);
    assert!(e["source"].as_str().unwrap().starts_with("unknown"), "{e}");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn requires_python_s_lower_bound_is_the_target() {
    let d = project("pyproject", "", Some(">=3.10,<4"));
    assert_eq!(finding(&d, "app.py").0, "auto");
    run(&d, &["fix", "app.py"]);
    assert!(fs::read_to_string(d.join("app.py")).unwrap().contains("@functools.cache\n"));
    let e = printed(&d, "targets.python");
    assert_eq!(e["value"], "3.10");
    assert!(e["source"].as_str().unwrap().starts_with("pyproject.toml:3"), "{e}");
    let why = String::from_utf8(run(&d, &["rules", "--why", RULE]).stdout).unwrap();
    assert!(why.contains("fix target: needs python 3.9; target python 3.10 (pyproject.toml:3"), "{why}");
    let _ = fs::remove_dir_all(&d);
    // Below the rule's version: `suggest`, naming the target.
    let d = project("old", "", Some(">=3.8"));
    let (fix, note) = finding(&d, "app.py");
    assert_eq!(fix, "suggest");
    assert!(note.contains("the target is 3.8"), "{note}");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn the_config_s_targets_are_over_pyproject_and_scopes_and_entries_over_those() {
    let config = "\n[targets]\npython = \"3.9\"\n\n[scope.legacy]\nfiles = [\"legacy/**\"]\ntargets = { python = \"3.8\" }\n";
    let d = project("config", config, Some(">=3.8"));
    assert_eq!(finding(&d, "app.py").0, "auto", "[targets] python is over requires-python");
    assert_eq!(finding(&d, "legacy/old.py").0, "suggest", "the scope's target is over [targets]");
    let _ = fs::remove_dir_all(&d);
    let entry = format!("\n[scope.legacy]\nfiles = [\"legacy/**\"]\ntargets = {{ python = \"3.8\" }}\nrules = {{ \"{RULE}\" = {{ targets = {{ python = \"3.12\" }} }} }}\n");
    let d = project("entry", &entry, None);
    assert_eq!(finding(&d, "legacy/old.py").0, "auto", "the rule entry's target is over its scope's");
    assert_eq!(finding(&d, "app.py").0, "suggest");
    let _ = fs::remove_dir_all(&d);
}

#[test]
fn a_target_that_is_not_one_is_a_config_error() {
    for bad in ["\n[targets]\npython = \"three\"\n", "\n[targets]\nbrowsers = [\"opera >= 90\"]\n", "\n[targets]\nbrowsers = [\"last 2 versions\"]\n"] {
        let d = project("bad", bad, None);
        let out = run(&d, &["check", "app.py"]);
        assert_eq!(out.status.code(), Some(2), "{bad}: {}", String::from_utf8_lossy(&out.stderr));
        let _ = fs::remove_dir_all(&d);
    }
}

#[test]
fn a_browserslist_counts_only_when_every_query_is_a_minimum() {
    let d = project("browserslist", "", None);
    fs::write(d.join("package.json"), r#"{ "browserslist": ["chrome >= 111", "safari >= 16.4"] }"#).unwrap();
    let e = printed(&d, "targets.browsers");
    assert_eq!(e["value"], serde_json::json!(["chrome >= 111", "safari >= 16.4"]));
    fs::write(d.join("package.json"), r#"{ "browserslist": ["defaults", "chrome >= 111"] }"#).unwrap();
    let e = printed(&d, "targets.browsers");
    assert_eq!(e["value"], Value::Null);
    assert!(e["source"].as_str().unwrap().contains("`defaults` is not `<browser> >= <version>`"), "{e}");
    fs::write(d.join("codelaw.toml"), "extends = [\"codelaw:py/recommended\"]\n\n[targets]\nbrowsers = [\"firefox >= 113\"]\n").unwrap();
    assert_eq!(printed(&d, "targets.browsers")["value"], serde_json::json!(["firefox >= 113"]));
    let _ = fs::remove_dir_all(&d);
}

/// A CSS project with the css pack, `config` appended, and the browser
/// table installed when `table` is set.
fn css_project(name: &str, config: &str, table: bool) -> PathBuf {
    let d = temp(name);
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf();
    copy_dir(&repo.join("packs/css"), &d.join("node_modules/@codelaw/rules-css"));
    if table {
        copy_dir(&repo.join("data-packages/browsers"), &d.join("node_modules/@codelaw/data-browsers"));
    }
    fs::write(d.join("codelaw.toml"), format!("extends = [\"codelaw:css/recommended\"]\npreview = true\n\n[rules]\n\"css/property-no-vendor-prefix\" = \"warn\"\n{config}")).unwrap();
    fs::write(d.join("a.css"), "a {\n  -webkit-mask-image: none;\n}\n").unwrap();
    d
}

fn css_finding(d: &Path) -> (String, String) {
    let out = run(d, &["check", "a.css", "--format", "json", "--no-cache"]);
    let v: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out.stderr)));
    let d = v["diagnostics"].as_array().unwrap().iter().find(|d| d["rule"] == "css/property-no-vendor-prefix").unwrap_or_else(|| panic!("{v:#}"));
    (d["fix"]["class"].as_str().unwrap_or_default().to_owned(), d["fix"]["note"].as_str().unwrap_or_default().to_owned())
}

#[test]
fn a_prefix_fix_is_auto_where_the_table_says_every_target_browser_supports_it() {
    let targets = "\n[targets]\nbrowsers = [\"chrome >= 120\", \"firefox >= 53\", \"safari >= 15.4\"]\n";
    let d = css_project("css-auto", targets, true);
    assert_eq!(css_finding(&d).0, "auto");
    let v: Value = serde_json::from_slice(&run(&d, &["--version", "--json"]).stdout).unwrap();
    assert!(v["data"]["browsers"]["version"].as_str().is_some_and(|s| s.starts_with("20")), "{v}");
    let _ = fs::remove_dir_all(&d);
    // A target below the row's version: `suggest`, naming both.
    let d = css_project("css-old", "\n[targets]\nbrowsers = [\"safari >= 15\"]\n", true);
    let (fix, note) = css_finding(&d);
    assert_eq!(fix, "suggest");
    assert!(note.contains("`css-property:mask-image` is unprefixed from safari 15.4"), "{note}");
    let _ = fs::remove_dir_all(&d);
    // No table: `suggest`, saying how to install it.
    let d = css_project("css-none", targets, false);
    let (fix, note) = css_finding(&d);
    assert_eq!(fix, "suggest");
    assert!(note.contains("@codelaw/data-browsers"), "{note}");
    let v: Value = serde_json::from_slice(&run(&d, &["--version", "--json"]).stdout).unwrap();
    assert_eq!(v["data"]["browsers"], Value::Null);
    let _ = fs::remove_dir_all(&d);
}
