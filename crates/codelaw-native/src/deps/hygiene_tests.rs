//! W-129 DoD 1–4: `deps/licence`, `deps/lockfile-sync`,
//! `deps/version-drift` and `deps/runtime-agreement` on small repos.

use super::{Licence, LockfileSync, RuntimeAgreement, VersionDrift};
use crate::{Elsewhere, FileContext, NativeContext, NativeFinding, NativeRule};
use codelaw_core::{Language, Severity};
use serde_json::{Value, json};
use std::collections::BTreeMap;

struct Repo {
    path: String,
    files: BTreeMap<String, String>,
    as_of: Option<String>,
}

impl FileContext for Repo {
    fn path(&self) -> &str {
        &self.path
    }
    fn bytes(&self) -> &[u8] {
        self.files[&self.path].as_bytes()
    }
    fn exists(&self, path: &str) -> bool {
        self.files.contains_key(path) || self.is_dir(path)
    }
    fn is_dir(&self, path: &str) -> bool {
        let d = format!("{path}/");
        path.is_empty() || self.files.keys().any(|p| p.starts_with(&d))
    }
    fn read(&self, path: &str) -> Option<String> {
        self.files.get(path).cloned()
    }
    fn list(&self, path: &str) -> Option<Vec<String>> {
        let d = if path.is_empty() { String::new() } else { format!("{path}/") };
        let mut out: Vec<String> = self.files.keys().filter_map(|p| p.strip_prefix(&d)).map(|r| r.split('/').next().unwrap_or(r).to_owned()).collect();
        out.dedup();
        (!out.is_empty()).then_some(out)
    }
    fn project_files(&self) -> Option<Vec<String>> {
        Some(self.files.keys().cloned().collect())
    }
    fn as_of(&self) -> Option<String> {
        self.as_of.clone()
    }
}

/// One finding: its message, severity, the text it covers, the file after
/// its edits, and its copies elsewhere.
#[derive(Debug)]
struct Got {
    message: String,
    severity: Option<Severity>,
    at: String,
    fixed: String,
    elsewhere: Vec<Elsewhere>,
}

fn run(rule: &dyn NativeRule, path: &str, files: &[(&str, &str)], options: Value) -> Vec<Got> {
    let files: BTreeMap<String, String> = files.iter().map(|(p, t)| (p.to_string(), t.to_string())).collect();
    let src = files[path].clone();
    let repo = Repo { path: path.into(), files, as_of: Some("2026-10-30".into()) };
    let lang = if path.ends_with(".json") {
        Language::Json
    } else if path.ends_with(".toml") {
        Language::Toml
    } else {
        Language::Yaml
    };
    let root = codelaw_lang::parse(lang, &src);
    let options: BTreeMap<String, Value> = options.as_object().cloned().unwrap_or_default().into_iter().collect();
    let data = BTreeMap::new();
    let ctx = NativeContext { source: &src, language: lang, semantic: None, options: &options, data: &data, file: &repo };
    let mut out: Vec<NativeFinding> = Vec::new();
    for node in root.root().dfs() {
        if rule.kinds().contains(&node.kind().as_ref()) {
            rule.check(node, &ctx, &mut out);
        }
    }
    out.into_iter()
        .map(|f| {
            let mut fixed = src.clone();
            for e in f.edits.iter().rev() {
                fixed.replace_range(e.start..e.end, &e.text);
            }
            Got { severity: rule.severity_of(&f, &options), at: src[f.start..f.end].to_owned(), fixed, elsewhere: rule.elsewhere(&f, &ctx), message: f.message }
        })
        .collect()
}

// ---- deps/licence -------------------------------------------------------

const LICENCE_SNAPSHOT: &str = r#"{
  "fetched": "2026-10-29",
  "packages": {
    "npm:left": { "versions": { "1.0.0": "" }, "licence": { "1.0.0": "GPL-3.0-only" } },
    "npm:right": { "versions": { "2.0.0": "" }, "licence": { "2.0.0": "MIT OR GPL-3.0-only" } },
    "npm:odd": { "versions": { "1.1.0": "" }, "licence": { "1.1.0": "see LICENSE" } },
    "npm:mpl": { "versions": { "3.0.0": "" }, "licence": { "3.0.0": "MPL-2.0" } },
    "npm:none": { "versions": { "1.0.0": "" } },
    "npm:deep": { "licence": { "0.4.0": "AGPL-3.0-or-later" } },
    "pypi:lgpl-thing": { "versions": { "2.1": "" }, "licence": { "2.1": "GNU Lesser General Public License v3 (LGPLv3)" } }
  },
  "sources": { "osv": { "kind": "osv", "state": "ok", "on": "2026-10-29" } }
}"#;

const LICENCE_MANIFEST: &str = r#"{
  "name": "app",
  "dependencies": { "left": "^1.0.0", "right": "^2.0.0", "odd": "^1.1.0", "mpl": "^3.0.0", "none": "^1.0.0" }
}"#;

const LICENCE_LOCK: &str = r#"{"lockfileVersion":3,"packages":{"":{"dependencies":{"left":"^1.0.0","right":"^2.0.0","odd":"^1.1.0","mpl":"^3.0.0","none":"^1.0.0"}},
"node_modules/left":{"version":"1.0.0"},"node_modules/right":{"version":"2.0.0"},"node_modules/odd":{"version":"1.1.0"},"node_modules/mpl":{"version":"3.0.0"},"node_modules/none":{"version":"1.0.0"},"node_modules/deep":{"version":"0.4.0"}}}"#;

#[test]
fn licence_direct_transitive_and_grades() {
    let files = [("package.json", LICENCE_MANIFEST), ("package-lock.json", LICENCE_LOCK), (".codelaw/deps-snapshot.json", LICENCE_SNAPSHOT)];
    let got = run(&Licence, "package.json", &files, json!({}));
    let lines: Vec<(String, Option<Severity>)> = got.iter().map(|g| (g.message.split(" (").next().unwrap_or("").chars().take(60).collect(), g.severity)).collect();
    assert_eq!(got.len(), 5, "{lines:#?}");
    assert!(got[0].message.starts_with("`left` 1.0.0 is licensed `GPL-3.0-only`, which `[deps.licence]` denies") && got[0].message.ends_with("(denied)"), "{}", got[0].message);
    assert_eq!((got[0].at.as_str(), got[0].severity), ("^1.0.0", None));
    assert!(got[1].message.contains("`see LICENSE`, which is not an SPDX expression") && got[1].severity == Some(Severity::Warn), "{}", got[1].message);
    assert!(got[2].message.contains("`MPL-2.0`, which `[deps.licence]` neither allows nor denies"), "{}", got[2].message);
    assert!(got[3].message.starts_with("`none` 1.0.0 names no licence in the registry"), "{}", got[3].message);
    assert!(got[4].message.starts_with("1 package (installed through another package, in package-lock.json) has a licence `[deps.licence]` does not allow: `deep` 0.4.0 (`AGPL-3.0-or-later`, denied); replace"), "{}", got[4].message);
    assert!(got[4].message.ends_with("(denied)"));
    assert_eq!(got[4].at, "{");
    assert!(got.iter().all(|g| g.fixed.is_empty() || g.fixed == LICENCE_MANIFEST));

    // Configured: MPL allowed, unknown off, `left` ignored until a date to come.
    let opts = json!({ "licence": { "allow": ["MPL-*", "AGPL-*"], "unknown": "off", "ignore": [{ "package": "npm:left", "expires": "2026-12-01", "reason": "replacing it" }] } });
    let got = run(&Licence, "package.json", &files, opts);
    assert!(got.is_empty(), "{got:#?}");
    // An expired ignore holds no longer; `unknown = "error"` grades the unknowns.
    let opts = json!({ "licence": { "unknown": "error", "ignore": [{ "package": "npm:left", "expires": "2026-10-01" }] } });
    let got = run(&Licence, "package.json", &files, opts);
    assert_eq!(got.len(), 5);
    assert_eq!(got[1].severity, Some(Severity::Error));
}

#[test]
fn licence_pypi_classifier_and_yarn_copy() {
    let snap = LICENCE_SNAPSHOT;
    let py = "[project]\nname = \"app\"\ndependencies = [\"lgpl-thing>=2.1\"]\n";
    let got = run(&Licence, "pyproject.toml", &[("pyproject.toml", py), (".codelaw/deps-snapshot.json", snap)], json!({}));
    assert_eq!(got.len(), 1);
    assert!(got[0].message.contains("denies") && got[0].at == "lgpl-thing>=2.1", "{got:#?}");
    let yarn = "# yarn lockfile v1\n\n\"left@^1.0.0\":\n  version \"1.0.0\"\n\ndeep@^0.4.0:\n  version \"0.4.0\"\n";
    let pj = r#"{ "dependencies": { "left": "^1.0.0" } }"#;
    let got = run(&Licence, "package.json", &[("package.json", pj), ("yarn.lock", yarn), (".codelaw/deps-snapshot.json", snap)], json!({}));
    assert_eq!(got.len(), 2, "{got:#?}");
    assert_eq!(got[1].elsewhere.len(), 1);
    assert_eq!(got[1].elsewhere[0].path, "yarn.lock");
    assert_eq!(&yarn[got[1].elsewhere[0].start..got[1].elsewhere[0].end], "deep@^0.4.0:");
    // A snapshot from before licences were kept: nothing judged.
    let old = r#"{ "fetched": "2026-10-01", "packages": { "npm:left": { "versions": { "1.0.0": "" } } } }"#;
    assert!(run(&Licence, "package.json", &[("package.json", pj), (".codelaw/deps-snapshot.json", old)], json!({})).is_empty());
}

// ---- deps/lockfile-sync -------------------------------------------------

#[test]
fn package_lock_and_pnpm_out_of_sync() {
    let pj = r#"{ "name": "app", "dependencies": { "react": "^18.3.1", "added": "^1.0.0" }, "devDependencies": { "vitest": "^2.0.0" } }"#;
    let lock = r#"{"lockfileVersion":3,"packages":{"":{"dependencies":{"react":"^18.2.0","gone":"^3.0.0"},"devDependencies":{"vitest":"^2.0.0"}}}}"#;
    let got = run(&LockfileSync, "package.json", &[("package.json", pj), ("package-lock.json", lock)], json!({}));
    let msgs: Vec<&str> = got.iter().map(|g| g.message.as_str()).collect();
    assert_eq!(got.len(), 3, "{msgs:#?}");
    assert!(msgs[0].starts_with("`react` ^18.3.1 — package-lock.json locked it as ^18.2.0: run `npm install`"), "{}", msgs[0]);
    assert_eq!(got[0].at, "^18.3.1");
    assert!(msgs[1].starts_with("`added` ^1.0.0 is not in package-lock.json"));
    assert!(msgs[2].starts_with("package-lock.json still locks `gone` ^3.0.0, which the manifest no longer names"));
    let in_sync = r#"{"lockfileVersion":3,"packages":{"":{"dependencies":{"react":"^18.3.1","added":"^1.0.0"},"devDependencies":{"vitest":"^2.0.0"}}}}"#;
    assert!(run(&LockfileSync, "package.json", &[("package.json", pj), ("package-lock.json", in_sync)], json!({})).is_empty());

    // A workspace's entry in the root lockfile; a folder with none is not one.
    let ws = r#"{"packages":{"":{"workspaces":["packages/*"]},"packages/web":{"dependencies":{"react":"^18.2.0"}}}}"#;
    let web = r#"{ "dependencies": { "react": "^18.3.1" } }"#;
    let got = run(&LockfileSync, "packages/web/package.json", &[("packages/web/package.json", web), ("package-lock.json", ws)], json!({}));
    assert_eq!(got.len(), 1);
    assert!(got[0].message.contains("(`package-lock.json`)"));
    assert!(run(&LockfileSync, "fixtures/x/package.json", &[("fixtures/x/package.json", web), ("package-lock.json", ws)], json!({})).is_empty());

    let pnpm = "lockfileVersion: '9.0'\nimporters:\n  .:\n    dependencies:\n      react:\n        specifier: ^18.2.0\n        version: 18.3.1\n";
    let pj = r#"{ "dependencies": { "react": "^18.3.1" }, "peerDependencies": { "x": "*" } }"#;
    let got = run(&LockfileSync, "package.json", &[("package.json", pj), ("pnpm-lock.yaml", pnpm)], json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].message.contains("pnpm-lock.yaml locked it as ^18.2.0: run `pnpm install`"));
}

#[test]
fn yarn_and_uv_out_of_sync() {
    let yarn = "# yarn lockfile v1\n\n\"react@^18.2.0\":\n  version \"18.3.1\"\n";
    let pj = r#"{ "dependencies": { "react": "^18.3.1", "w": "workspace:*" } }"#;
    let got = run(&LockfileSync, "package.json", &[("package.json", pj), ("yarn.lock", yarn)], json!({}));
    assert_eq!(got.len(), 1);
    assert!(got[0].message.starts_with("`react` ^18.3.1 is not in yarn.lock: run `yarn install`"));
    let berry = "__metadata:\n  version: 8\n\n\"react@npm:^18.3.1\":\n  version: 18.3.1\n";
    assert!(run(&LockfileSync, "package.json", &[("package.json", pj), ("yarn.lock", berry)], json!({})).is_empty());

    let py = "[project]\nname = \"App\"\ndependencies = [\"requests>=2.32\", \"rich\"]\n\n[dependency-groups]\nlint = [\"ruff>=0.6\"]\n";
    let uv = "version = 1\n\n[[package]]\nname = \"app\"\nversion = \"0.1.0\"\nsource = { virtual = \".\" }\n\n[package.metadata]\nrequires-dist = [\n    { name = \"requests\", specifier = \">=2.31\" },\n    { name = \"click\" },\n]\n\n[package.metadata.requires-dev]\nlint = [{ name = \"ruff\", specifier = \">=0.6.0\" }]\n";
    let got = run(&LockfileSync, "pyproject.toml", &[("pyproject.toml", py), ("uv.lock", uv)], json!({}));
    let msgs: Vec<&str> = got.iter().map(|g| g.message.as_str()).collect();
    assert_eq!(got.len(), 3, "{msgs:#?}");
    assert!(msgs[0].starts_with("`requests>=2.32` — uv.lock locked `requests` as >=2.31: run `uv lock`"), "{}", msgs[0]);
    assert!(msgs[1].starts_with("`rich` is not in uv.lock's requires-dist"));
    assert!(msgs[2].starts_with("uv.lock still locks `click` for the project"));
}

/// The content-hashes below are Poetry's algorithm run in Python
/// (`json.dumps(..., sort_keys=True)`, SHA-256) on the same pyproject.
#[test]
fn poetry_content_hash() {
    let v1 = "[tool.poetry]\nname = \"app\"\nversion = \"0.1.0\"\n\n[tool.poetry.dependencies]\npython = \"^3.10\"\nrequests = { version = \"^2.31\", extras = [\"socks\"] }\n\"caf\\u00e9\" = \"1.0\"\n\n[tool.poetry.group.dev.dependencies]\npytest = \"^8.0\"\n";
    let lock = |h: &str| format!("[[package]]\nname = \"requests\"\nversion = \"2.32.3\"\n\n[metadata]\nlock-version = \"2.0\"\npython-versions = \"^3.10\"\ncontent-hash = \"{h}\"\n");
    let ok = lock("315947a4bbca31c00e0f2543028e0e6abccfca0c1c5bbea344dfb71f86b4bd43");
    assert!(run(&LockfileSync, "pyproject.toml", &[("pyproject.toml", v1), ("poetry.lock", &ok)], json!({})).is_empty());
    let stale = lock("0000000000000000000000000000000000000000000000000000000000000000");
    let got = run(&LockfileSync, "pyproject.toml", &[("pyproject.toml", v1), ("poetry.lock", &stale)], json!({}));
    assert_eq!(got.len(), 1);
    assert!(got[0].message.starts_with("poetry.lock's content-hash does not match pyproject.toml's dependencies") && got[0].message.contains("run `poetry lock`"));
    let v2 = "[project]\nname = \"app\"\nversion = \"0.1.0\"\nrequires-python = \">=3.10\"\ndependencies = [\"requests>=2.31\"]\n\n[tool.poetry.group.dev.dependencies]\npytest = \"^8.0\"\n";
    let ok2 = lock("079e09cb567d8c12c24a21f75cdeb15848c9acd37bb6029745515bd1f2da822c");
    assert!(run(&LockfileSync, "pyproject.toml", &[("pyproject.toml", v2), ("poetry.lock", &ok2)], json!({})).is_empty());
}

// ---- deps/version-drift -------------------------------------------------

#[test]
fn npm_workspaces_drift() {
    let root = r#"{ "name": "mono", "private": true, "workspaces": ["packages/*"], "devDependencies": { "typescript": "^5.4.0" } }"#;
    let a = r#"{ "name": "a", "dependencies": { "react": "^18.2.0" }, "devDependencies": { "typescript": "^5.6.2" } }"#;
    let b = r#"{ "name": "b", "dependencies": { "react": "^18.3.1" }, "peerDependencies": { "react": "^17 || ^18" } }"#;
    let files = [("package.json", root), ("packages/a/package.json", a), ("packages/b/package.json", b)];
    let got = run(&VersionDrift, "packages/a/package.json", &files, json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].message.starts_with("`react` is ^18.2.0 here but ^18.3.1 in `packages/b/package.json` across the workspace: align it to ^18.3.1, the highest"), "{}", got[0].message);
    assert!(got[0].fixed.contains(r#""react": "^18.3.1""#));
    // The root's range is lower: under `highest` it moves up, under `root` the members move down.
    let got = run(&VersionDrift, "package.json", &files, json!({}));
    assert_eq!(got.len(), 1);
    assert!(got[0].fixed.contains(r#""typescript": "^5.6.2""#));
    let got = run(&VersionDrift, "packages/a/package.json", &files, json!({ "drift": { "policy": "root" } }));
    assert_eq!(got.len(), 2);
    assert!(got[1].fixed.contains(r#""typescript": "^5.4.0""#) && got[1].message.contains("the root's"));
    assert!(run(&VersionDrift, "packages/a/package.json", &files, json!({ "drift": { "ignore": ["npm:react"] } })).is_empty());
    // b agrees with the highest; its peer range is not compared.
    assert!(run(&VersionDrift, "packages/b/package.json", &files, json!({})).is_empty());
    // Not a workspace member: nothing.
    assert!(run(&VersionDrift, "tools/package.json", &[("package.json", root), ("packages/a/package.json", a), ("tools/package.json", a)], json!({})).is_empty());
}

#[test]
fn pnpm_and_uv_workspaces_drift() {
    let root = r#"{ "name": "mono" }"#;
    let ws = "packages:\n  - 'apps/**'\n  - '!apps/legacy'\n";
    let a = r#"{ "dependencies": { "zod": "3.22.0" } }"#;
    let b = r#"{ "dependencies": { "zod": "3.23.8" } }"#;
    let files = [("package.json", root), ("pnpm-workspace.yaml", ws), ("apps/web/package.json", a), ("apps/api/x/package.json", b), ("apps/legacy/package.json", r#"{ "dependencies": { "zod": "4.0.0" } }"#)];
    let got = run(&VersionDrift, "apps/web/package.json", &files, json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].fixed.contains(r#""zod": "3.23.8""#));

    let root = "[project]\nname = \"mono\"\ndependencies = [\"httpx>=0.27\"]\n\n[tool.uv.workspace]\nmembers = [\"libs/*\"]\n";
    let lib = "[project]\nname = \"lib\"\ndependencies = [\"HTTPX >= 0.27.0\", \"pydantic>=2.5 ; python_version >= '3.10'\"]\n";
    let lib2 = "[project]\nname = \"lib2\"\ndependencies = [\"pydantic>=2.8\"]\n";
    let files = [("pyproject.toml", root), ("libs/a/pyproject.toml", lib), ("libs/b/pyproject.toml", lib2)];
    let got = run(&VersionDrift, "libs/a/pyproject.toml", &files, json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].message.starts_with("`pydantic` is >=2.5 here but >=2.8 in `libs/b/pyproject.toml`"), "{}", got[0].message);
    assert!(got[0].fixed.contains("\"pydantic>=2.8 ; python_version >= '3.10'\""), "{}", got[0].fixed);
}

// ---- deps/runtime-agreement ---------------------------------------------

#[test]
fn node_pins_against_engines() {
    let pj = r#"{ "name": "app", "engines": { "node": ">=20.11" }, "volta": { "node": "18.19.0" } }"#;
    let wf = "on: push\njobs:\n  t:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/setup-node@v4\n        with:\n          node-version: 18\n";
    let files = [("package.json", pj), (".nvmrc", "v20.11.1\n"), (".tool-versions", "nodejs 22.1.0\n"), ("Dockerfile", "FROM node:18-alpine\n"), (".github/workflows/ci.yml", wf)];
    let got = run(&RuntimeAgreement, "package.json", &files, json!({}));
    let msgs: Vec<&str> = got.iter().map(|g| g.message.as_str()).collect();
    assert_eq!(got.len(), 3, "{msgs:#?}");
    // volta: in package.json, an edit to the agreed version.
    assert!(msgs[0].starts_with("Node 18.19.0 in `volta.node` in package.json is outside `engines.node` in package.json `>=20.11`: set it to 20.11.1"), "{}", msgs[0]);
    assert!(got[0].fixed.contains(r#""node": "20.11.1""#));
    // .tool-versions agrees with the range but not with .nvmrc; Dockerfile is outside.
    assert!(msgs[1].starts_with("Node 22.1.0 in `.tool-versions` (line 1) does not agree with v20.11.1 in `.nvmrc` (line 1): set it to 20.11.1 (by hand"), "{}", msgs[1]);
    assert_eq!(got[1].at, ">=20.11");
    assert_eq!(got[1].elsewhere.len(), 1);
    assert_eq!((got[1].elsewhere[0].path.as_str(), got[1].elsewhere[0].start), (".tool-versions", 0));
    assert!(msgs[2].starts_with("Node 18-alpine in `FROM node:18-alpine` in `Dockerfile` (line 1) is outside"), "{}", msgs[2]);
    assert_eq!(got[2].elsewhere[0].path, "Dockerfile");
    // The workflow's pin is reported in the workflow, with an edit.
    let got = run(&RuntimeAgreement, ".github/workflows/ci.yml", &files, json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert_eq!(got[0].at, "18");
    assert!(got[0].fixed.ends_with("node-version: 20\n"), "{}", got[0].fixed);
    assert!(got[0].elsewhere.is_empty());
}

#[test]
fn configured_truth_python_and_mise() {
    let pj = r#"{ "engines": { "node": ">=18 <20" } }"#;
    let files = [("package.json", pj), (".nvmrc", "20\n")];
    let got = run(&RuntimeAgreement, "package.json", &files, json!({ "runtime": { "source": { "node": ".nvmrc" } } }));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].message.starts_with("`engines.node` in package.json `>=18 <20` does not allow Node 20, which `.nvmrc` (line 1) names"), "{}", got[0].message);
    assert!(got[0].elsewhere.is_empty() && got[0].fixed == pj);

    let py = "[project]\nname = \"a\"\nrequires-python = \">=3.11\"\n";
    let mise = "[tools]\npython = \"3.10\"\nnode = \"20\"\n";
    let files = [("pyproject.toml", py), (".python-version", "3.12\n"), ("mise.toml", mise)];
    let got = run(&RuntimeAgreement, "mise.toml", &files, json!({}));
    assert_eq!(got.len(), 1, "{got:#?}");
    assert!(got[0].message.starts_with("Python 3.10 in `[tools] python` in mise.toml is outside `requires-python` in pyproject.toml `>=3.11`: set it to 3.12"));
    assert!(got[0].fixed.contains("python = \"3.12\""));
    assert!(run(&RuntimeAgreement, "pyproject.toml", &files, json!({})).is_empty());
    // One source alone agrees with itself.
    assert!(run(&RuntimeAgreement, "package.json", &[("package.json", pj)], json!({})).is_empty());
}
