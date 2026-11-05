//! W-146 through the binary and real git: `git/commit-message` in the
//! `commit-msg` hook and over a `--changed-since` range, `git/branch-name`,
//! `git/pr-size`, `git/pr-description` from each CI's own data, and the agent
//! hook's `git commit -m`.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Every variable the `git/*` rules read: a test that runs inside a CI must
/// not see that CI's pull request.
const CI_VARS: &[&str] = &[
    "GITHUB_EVENT_PATH",
    "GITHUB_HEAD_REF",
    "GITLAB_CI",
    "CI_MERGE_REQUEST_IID",
    "CI_MERGE_REQUEST_TITLE",
    "CI_MERGE_REQUEST_DESCRIPTION",
    "CI_MERGE_REQUEST_SOURCE_BRANCH_NAME",
    "CI_COMMIT_BRANCH",
    "BITBUCKET_BRANCH",
    "BITBUCKET_PR_ID",
    "SYSTEM_PULLREQUEST_PULLREQUESTID",
    "SYSTEM_PULLREQUEST_SOURCEBRANCH",
    "BUILD_SOURCEBRANCH",
];

fn have_git() -> bool {
    Command::new("git").arg("--version").output().is_ok_and(|o| o.status.success()) && Command::new("sh").arg("-c").arg("true").status().is_ok_and(|s| s.success())
}

/// A scratch repo on `main` with `config` and one commit.
fn temp(name: &str, config: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("codelaw-pr-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(d.join(".codelaw/rules")).unwrap();
    let d = fs::canonicalize(d).unwrap();
    fs::write(d.join(".codelaw/rules/README.md"), "local rules\n").unwrap();
    fs::write(d.join("codelaw.toml"), format!("extends = []\n{config}")).unwrap();
    fs::write(d.join(".gitignore"), ".codelaw/cache/\n").unwrap();
    git(&d, &["init", "-q", "-b", "main"]);
    git(&d, &["add", "."]);
    git(&d, &["commit", "-q", "-m", "chore: base"]);
    d
}

fn path_with_codelaw() -> String {
    let bin = Path::new(env!("CARGO_BIN_EXE_codelaw")).parent().unwrap().to_path_buf();
    format!("{}:{}", bin.display(), std::env::var("PATH").unwrap_or_default())
}

fn run(cmd: &mut Command, cwd: &Path, env: &[(&str, &str)]) -> (i32, String) {
    for k in CI_VARS {
        cmd.env_remove(k);
    }
    let out = cmd
        .current_dir(cwd)
        .env("PATH", path_with_codelaw())
        .env("NO_COLOR", "1")
        .env("CODELAW_FORMAT", "text")
        .env_remove("CI")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@example.com")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@example.com")
        .env_remove("CODELAW_CACHE_DIR")
        .envs(env.iter().copied())
        .output()
        .unwrap();
    (out.status.code().unwrap_or(-1), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

fn git(cwd: &Path, args: &[&str]) -> (i32, String) {
    run(Command::new("git").args(args), cwd, &[])
}

fn codelaw(cwd: &Path, args: &[&str], env: &[(&str, &str)]) -> (i32, String) {
    run(Command::new(env!("CARGO_BIN_EXE_codelaw")).args(args), cwd, env)
}

fn commit(d: &Path, file: &str, text: &str, message: &str) {
    fs::write(d.join(file), text).unwrap();
    git(d, &["add", "."]);
    let (code, out) = git(d, &["commit", "-q", "--no-verify", "-m", message]);
    assert_eq!(code, 0, "{out}");
}

fn json_check(d: &Path, env: &[(&str, &str)]) -> Value {
    let (code, out) = codelaw(d, &["check", "--format", "json", "--changed-since", "main"], env);
    assert!(code < 2, "{out}");
    serde_json::from_str(&out).unwrap_or_else(|e| panic!("{e}: {out}"))
}

fn findings<'a>(v: &'a Value, rule: &str) -> Vec<&'a Value> {
    v["diagnostics"].as_array().unwrap().iter().filter(|d| d["rule"] == rule).collect()
}

fn warnings(v: &Value) -> String {
    v["warnings"].as_array().map(|w| w.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n")).unwrap_or_default()
}

#[test]
fn the_commit_msg_hook_refuses_a_message_and_shows_one_that_passes() {
    if !have_git() {
        return;
    }
    let d = temp("hook", "");
    let (code, out) = codelaw(&d, &["init", "--git-hooks", "commit-msg"], &[]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("commit-msg: installed"), "{out}");
    let hook = fs::read_to_string(d.join(".git/hooks/commit-msg")).unwrap();
    assert!(hook.contains("# codelaw-hook: commit-msg"));
    fs::write(d.join("a.txt"), "a\n").unwrap();
    git(&d, &["add", "."]);
    let (code, out) = git(&d, &["commit", "-q", "-m", "Added the login page."]);
    assert_ne!(code, 0, "refused: {out}");
    assert!(out.contains("git/commit-message"), "{out}");
    assert!(out.contains("Write it as: \"feat: added the login page\""), "{out}");
    let (code, out) = git(&d, &["commit", "-q", "-m", "feat: add the login page"]);
    assert_eq!(code, 0, "{out}");
    // A bare `--git-hooks` does not install it.
    let e = temp("hook-default", "");
    let (_, out) = codelaw(&e, &["init", "--git-hooks"], &[]);
    assert!(!out.contains("commit-msg"), "{out}");
    // `preset = "off"` lets anything through.
    fs::write(d.join("codelaw.toml"), "extends = []\n[commits]\npreset = \"off\"\n").unwrap();
    fs::write(d.join("b.txt"), "b\n").unwrap();
    git(&d, &["add", "."]);
    let (code, out) = git(&d, &["commit", "-q", "-m", "whatever"]);
    assert_eq!(code, 0, "{out}");
}

#[test]
fn check_changed_since_judges_the_range_commits_and_skips_fixups() {
    if !have_git() {
        return;
    }
    let d = temp("range", "");
    git(&d, &["checkout", "-q", "-b", "feat/login"]);
    commit(&d, "a.txt", "a\n", "feat: add a");
    commit(&d, "b.txt", "b\n", "Update b");
    commit(&d, "c.txt", "c\n", "fixup! feat: add a");
    let v = json_check(&d, &[]);
    let f = findings(&v, "git/commit-message");
    assert_eq!(f.len(), 1, "{v:#}");
    assert!(f[0]["file"].as_str().unwrap().starts_with("commit:"));
    assert!(f[0]["message"].as_str().unwrap().contains("Write it as: \"chore: update b\""), "{v:#}");
    assert_eq!(f[0]["fix"]["class"], "manual");
    assert_eq!(f[0]["severity"], "error");
    // No branch pattern: branch names are not checked; a small change is no size finding.
    assert!(findings(&v, "git/branch-name").is_empty() && findings(&v, "git/pr-size").is_empty(), "{v:#}");
    // SARIF: a commit is a logical location.
    let (_, out) = codelaw(&d, &["check", "--format", "sarif", "--changed-since", "main"], &[]);
    let s: Value = serde_json::from_str(&out).unwrap();
    let r = s["runs"][0]["results"].as_array().unwrap().iter().find(|r| r["ruleId"] == "git/commit-message").unwrap();
    assert_eq!(r["locations"][0]["logicalLocations"][0]["kind"], "commit", "{r:#}");
    // GitHub: no `file=` for a commit.
    let (_, out) = codelaw(&d, &["check", "--format", "github", "--changed-since", "main"], &[]);
    assert!(out.lines().any(|l| l.starts_with("::error title=git/commit-message::")), "{out}");
    // Without `--changed-since` nothing is judged; `off` turns it off.
    let (_, out) = codelaw(&d, &["check", "--format", "json"], &[]);
    assert!(!out.contains("git/commit-message"), "{out}");
    fs::write(d.join("codelaw.toml"), "extends = []\n[rules]\n\"git/commit-message\" = \"off\"\n").unwrap();
    assert!(findings(&json_check(&d, &[]), "git/commit-message").is_empty());
}

#[test]
fn a_preset_shares_the_git_rules_settings_and_the_project_table_wins() {
    if !have_git() {
        return;
    }
    let d = temp("preset", "");
    // A pack whose manifest name is first-party, holding only a preset that
    // sets the engine's `git/*` rules, as a team's own preset would.
    let pack = d.join("node_modules/@codelaw/rules-js");
    fs::create_dir_all(pack.join("presets/js")).unwrap();
    fs::write(pack.join("codelaw-pack.toml"), "name = \"@codelaw/rules-js\"\nversion = \"0.0.1\"\nmin_codelaw = \"0.1.0\"\nlanguages = [\"javascript\"]\npresets = [\"js/team\"]\n").unwrap();
    fs::write(
        pack.join("presets/js/team.toml"),
        "[rules]\n\"git/commit-message\" = { severity = \"warn\", options = { types = [\"feat\"], skip = { fixups = false } } }\n\"git/pr-size\" = { options = { max_lines = 3 } }\n",
    )
    .unwrap();
    fs::write(d.join(".gitignore"), ".codelaw/cache/\nnode_modules/\n").unwrap();
    fs::write(d.join("codelaw.toml"), "extends = [\"codelaw:js/team\"]\n").unwrap();
    commit(&d, "base.txt", "base\n", "chore: the team preset");
    git(&d, &["checkout", "-q", "-b", "feat/team"]);
    commit(&d, "a.txt", &"x\n".repeat(5), "fix: a");
    let v = json_check(&d, &[]);
    let f = findings(&v, "git/commit-message");
    assert_eq!(f.len(), 1, "the preset's types: `fix` is not one: {v:#}");
    assert_eq!(f[0]["severity"], "warn", "the preset's severity");
    assert_eq!(findings(&v, "git/pr-size").len(), 1, "the preset's `max_lines`: {v:#}");
    // The project's `[commits]` table wins over the preset key by key; the
    // preset's other keys stay.
    fs::write(d.join("codelaw.toml"), "extends = [\"codelaw:js/team\"]\n[commits]\ntypes = [\"feat\", \"fix\"]\n").unwrap();
    let v = json_check(&d, &[]);
    assert!(findings(&v, "git/commit-message").is_empty(), "{v:#}");
    assert_eq!(findings(&v, "git/pr-size").len(), 1, "{v:#}");
    // And its `[rules]` severity over the preset's.
    fs::write(d.join("codelaw.toml"), "extends = [\"codelaw:js/team\"]\n[rules]\n\"git/commit-message\" = \"error\"\n").unwrap();
    assert_eq!(findings(&json_check(&d, &[]), "git/commit-message")[0]["severity"], "error");
}

#[test]
fn branch_names_once_a_pattern_is_set() {
    if !have_git() {
        return;
    }
    let d = temp("branch", "[branches]\npattern = \"(feat|fix)/[a-z0-9-]+\"\n");
    git(&d, &["checkout", "-q", "-b", "Login_Page"]);
    commit(&d, "a.txt", "a\n", "feat: add a");
    let v = json_check(&d, &[]);
    let f = findings(&v, "git/branch-name");
    assert_eq!(f.len(), 1, "{v:#}");
    assert_eq!(f[0]["file"], "branch:Login_Page");
    assert!(f[0]["message"].as_str().unwrap().contains("\"feat/login-page\""), "{v:#}");
    // The CI's branch variable wins over git's.
    assert!(findings(&json_check(&d, &[("GITHUB_HEAD_REF", "fix/login")]), "git/branch-name").is_empty());
    // A protected branch is never judged.
    assert!(findings(&json_check(&d, &[("CI_COMMIT_BRANCH", "release/2")]), "git/branch-name").is_empty());
}

#[test]
fn pr_size_counts_changed_lines_without_lockfiles() {
    if !have_git() {
        return;
    }
    let d = temp("size", "[pr]\nmax_lines = 5\n");
    git(&d, &["checkout", "-q", "-b", "feat/size"]);
    commit(&d, "package-lock.json", &"{}\n".repeat(100), "chore: lock");
    let v = json_check(&d, &[]);
    assert!(findings(&v, "git/pr-size").is_empty(), "a lockfile is not counted: {v:#}");
    commit(&d, "a.txt", &"x\n".repeat(10), "feat: add a");
    let v = json_check(&d, &[]);
    let f = findings(&v, "git/pr-size");
    assert_eq!(f.len(), 1, "{v:#}");
    assert_eq!(f[0]["severity"], "warn");
    assert_eq!(f[0]["data"]["lines"], 10);
    assert_eq!(f[0]["data"]["excluded"], 1);
}

#[test]
fn pr_description_from_each_ci() {
    if !have_git() {
        return;
    }
    let d = temp("desc", "[pr]\nrequire_ticket = true\ntemplate_sections = [\"Summary\"]\n");
    git(&d, &["checkout", "-q", "-b", "feat/desc"]);
    commit(&d, "a.txt", "a\n", "feat: add a");
    let event = d.join("event.json");
    fs::write(&event, r###"{"pull_request":{"title":"Add a","body":"## Summary\n<!-- what and why -->\n"}}"###).unwrap();
    let v = json_check(&d, &[("GITHUB_EVENT_PATH", event.to_str().unwrap())]);
    let f = findings(&v, "git/pr-description");
    assert_eq!(f.len(), 1, "{v:#}");
    assert_eq!(f[0]["file"], "pr");
    let m = f[0]["message"].as_str().unwrap();
    assert!(m.contains("names a ticket") && m.contains("`Summary` section is empty"), "{m}");
    fs::write(&event, r###"{"pull_request":{"title":"ABC-1 Add a","body":"## Summary\nAdds a.\n"}}"###).unwrap();
    assert!(findings(&json_check(&d, &[("GITHUB_EVENT_PATH", event.to_str().unwrap())]), "git/pr-description").is_empty());
    // GitLab's variables.
    let v = json_check(&d, &[("CI_MERGE_REQUEST_TITLE", "Add a"), ("CI_MERGE_REQUEST_DESCRIPTION", "no sections")]);
    assert!(findings(&v, "git/pr-description")[0]["message"].as_str().unwrap().contains("no `Summary` section"), "{v:#}");
    // Bitbucket: not fetched, said so.
    let v = json_check(&d, &[("BITBUCKET_PR_ID", "7")]);
    assert!(findings(&v, "git/pr-description").is_empty());
    assert!(warnings(&v).contains("Bitbucket Pipelines") && warnings(&v).contains("not checked here"), "{v:#}");
}

#[test]
fn a_bad_git_config_is_a_load_error() {
    let d = temp("config", "[commits]\npreset = \"nope\"\n");
    let (code, out) = codelaw(&d, &["check"], &[]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("`[commits] preset`"), "{out}");
    fs::write(d.join("codelaw.toml"), "[branches]\npattern = \"(\"\n").unwrap();
    let (code, out) = codelaw(&d, &["check"], &[]);
    assert_eq!(code, 2, "{out}");
    assert!(out.contains("`[branches] pattern`"), "{out}");
}

#[test]
fn the_agent_hook_refuses_a_git_commit_whose_message_fails() {
    let d = temp("agent", "");
    let payload = |cmd: &str| serde_json::json!({ "cwd": d, "tool_name": "Bash", "tool_input": { "command": cmd } }).to_string();
    let pre = |cmd: &str| {
        let mut c = Command::new(env!("CARGO_BIN_EXE_codelaw"));
        c.args(["hook", "--pre", "--agent", "claude"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).current_dir(&d);
        for k in CI_VARS {
            c.env_remove(k);
        }
        let mut child = c.spawn().unwrap();
        use std::io::Write;
        child.stdin.take().unwrap().write_all(payload(cmd).as_bytes()).unwrap();
        String::from_utf8_lossy(&child.wait_with_output().unwrap().stdout).into_owned()
    };
    let out = pre("git add . && git commit -m 'Added login'");
    let v: Value = serde_json::from_str(&out).unwrap_or_else(|e| panic!("{e}: {out}"));
    assert_eq!(v["hookSpecificOutput"]["permissionDecision"], "deny");
    let reason = v["hookSpecificOutput"]["permissionDecisionReason"].as_str().unwrap();
    assert!(reason.contains("refused this commit") && reason.contains("feat: added login"), "{reason}");
    assert_eq!(pre("git commit -m 'feat: add login'"), "");
    assert_eq!(pre("ls -la"), "");
}
