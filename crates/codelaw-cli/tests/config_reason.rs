//! A rule turned `"off"` may say why (SR-CONFIG decision 11c, W-158), on
//! real projects: `reason` on an `"off"` entry in `[rules]`, a scope and a
//! profile; `[rules] require_reason` making an `"off"` without one a
//! `codelaw/off-without-reason` finding on its line, in `check` and `fix`;
//! the default leaving such a config clean; a `reason` on an entry that is
//! not `"off"` and `require_reason` in a nested config refused at load;
//! and `config --print` showing the reason and the switch.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-w158r-{name}-{}", std::process::id()));
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

/// A project with the js pack and `config` as its `codelaw.toml`.
fn project(name: &str, config: &str) -> PathBuf {
    let d = temp(name);
    let pack = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().join("packs/js");
    copy_dir(&pack, &d.join("node_modules/@codelaw/rules-js"));
    fs::write(d.join("codelaw.toml"), config).unwrap();
    fs::create_dir_all(d.join("src")).unwrap();
    fs::write(d.join("src/app.js"), "console.log(1);\n").unwrap();
    d
}

/// `rule severity file:line` for every engine finding about reasons.
fn offs(cwd: &Path, verb: &str) -> Vec<String> {
    let out = run(cwd, &[verb, "--format", "json", "--no-cache"]);
    let text = String::from_utf8(out.stdout).unwrap();
    let v: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{e}: {text}\n{}", String::from_utf8_lossy(&out.stderr)));
    let list = v["diagnostics"].as_array().or(v["check"]["diagnostics"].as_array()).unwrap_or_else(|| panic!("{v:#}"));
    let mut found: Vec<String> = list
        .iter()
        .filter(|d| d["rule"] == "codelaw/off-without-reason")
        .map(|d| {
            let line = d["range"]["start"]["line"].as_u64().or(d["line"].as_u64()).unwrap_or(0);
            format!("{} {} {}:{line}", d["rule"].as_str().unwrap(), d["severity"].as_str().unwrap(), d["file"].as_str().unwrap())
        })
        .collect();
    found.sort();
    found
}

const RULES: &str = "extends = [\"codelaw:js/recommended\"]\n\n[rules]\n\"js/no-console\" = { severity = \"off\", reason = \"CLI tool: console is the output\" }\n\"js/no-debugger\" = \"off\"\n\n[scope.tests]\nfiles = [\"test/**\"]\nrules = { \"js/no-var\" = \"off\" }\n\n[profile.ci.rules]\n\"js/eqeqeq\" = { severity = \"off\", reason = \"legacy\" }\n";

#[test]
fn require_reason_makes_an_unexplained_off_a_finding_on_its_line() {
    // The default: no finding, every existing config valid.
    let d = project("default", RULES);
    assert_eq!(offs(&d, "check"), Vec::<String>::new());
    // On: the two `"off"` entries without a reason, at their lines.
    let on = RULES.replacen("[rules]\n", "[rules]\nrequire_reason = true\n", 1);
    let d = project("on", &on);
    let want = ["codelaw/off-without-reason error codelaw.toml:6", "codelaw/off-without-reason error codelaw.toml:10"];
    let mut want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
    want.sort();
    assert_eq!(offs(&d, "check"), want);
    assert_eq!(offs(&d, "fix"), want, "`fix` reports them too and changes nothing");
    assert_eq!(fs::read_to_string(d.join("codelaw.toml")).unwrap(), on);
    // Given reasons, the findings go.
    let fixed = on
        .replace("\"js/no-debugger\" = \"off\"", "\"js/no-debugger\" = { severity = \"off\", reason = \"pausing is the point\" }")
        .replace("{ \"js/no-var\" = \"off\" }", "{ \"js/no-var\" = { severity = \"off\", reason = \"old tests\" } }");
    fs::write(d.join("codelaw.toml"), &fixed).unwrap();
    assert_eq!(offs(&d, "check"), Vec::<String>::new());
}

#[test]
fn config_print_shows_the_reason_and_the_switch() {
    let d = project("print", &RULES.replacen("[rules]\n", "[rules]\nrequire_reason = false\n", 1));
    let out = run(&d, &["config", "--print"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{text}{}", String::from_utf8_lossy(&out.stderr));
    assert!(text.lines().any(|l| l.starts_with("rules.\"js/no-console\".reason = \"CLI tool: console is the output\"") && l.contains("codelaw.toml:5")), "{text}");
    assert!(text.lines().any(|l| l.starts_with("rules.require_reason = false") && l.contains("codelaw.toml:4")), "{text}");
    assert!(!text.contains("rules.\"js/no-debugger\".reason"), "{text}");
}

#[test]
fn a_misplaced_reason_or_switch_is_a_config_error() {
    let d = project("not-off", "[rules]\n\"js/no-console\" = { severity = \"warn\", reason = \"why\" }\n");
    let out = run(&d, &["check", "--no-cache"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("rule `js/no-console`: `reason` says why a rule is off"), "{err}");

    let d = project("in-scope", "[scope.t]\nfiles = [\"t/**\"]\nrules = { require_reason = true }\n");
    let out = run(&d, &["check", "--no-cache"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("`require_reason` is set in the top-level `[rules]` only"), "{err}");

    let d = project("not-bool", "[rules]\nrequire_reason = \"yes\"\n");
    let out = run(&d, &["check", "--no-cache"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("`[rules] require_reason` is true or false"), "{err}");

    let d = project("nested", "extends = [\"codelaw:js/recommended\"]\n");
    fs::write(d.join("src/codelaw.toml"), "[rules]\nrequire_reason = true\n").unwrap();
    let out = run(&d, &["check", "--no-cache", "src/app.js"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "{err}");
    assert!(err.contains("[rules] require_reason can only be set in the root config"), "{err}");
}
