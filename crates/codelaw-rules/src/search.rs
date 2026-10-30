//! Where the engine finds a rule pack (SR-PACK-LOOKUP).
//!
//! The one lookup every caller uses (decision 9): the CLI, the LSP, the MCP
//! server, `codelaw migrate`, and the skills-package lookup's Python step.
//! Each pack is looked up on its own (decision 1), on the registries of
//! `[packs] order` (decision 2): npm's places, then Python's.

use crate::pack::first_party;
use codelaw_config::{Config, Registry, venv_site_packages};
use std::path::{Path, PathBuf};

/// The two variables the Python step reads (decision 4a and 4b). Read once,
/// by [`PackEnv::from_env`]; nothing else reads them to find a pack.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackEnv {
    /// `CODELAW_SITE_PACKAGES`, set by the Python launcher.
    pub site_packages: Option<String>,
    /// `VIRTUAL_ENV`.
    pub virtual_env: Option<String>,
}

impl PackEnv {
    pub fn from_env() -> Self {
        Self::from_vars(&|k| std::env::var(k).ok())
    }

    /// The same, from a variable lookup a test can stand in for.
    pub fn from_vars(var: &dyn Fn(&str) -> Option<String>) -> Self {
        let get = |k: &str| var(k).filter(|v| !v.is_empty());
        PackEnv { site_packages: get("CODELAW_SITE_PACKAGES"), virtual_env: get("VIRTUAL_ENV") }
    }
}

/// One folder a pack is looked for in, and why it is searched (decision 8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    pub registry: Registry,
    /// A `node_modules` folder, or a site-packages folder.
    pub dir: PathBuf,
    /// `[packs] node_modules`, `walk-up`, `[packs] venv`,
    /// `CODELAW_SITE_PACKAGES`, `VIRTUAL_ENV` or `.venv`.
    pub source: &'static str,
}

/// The places of one project, in the order they are searched.
#[derive(Clone, Debug, Default)]
pub struct PackSearch {
    pub order: Vec<Registry>,
    /// `node_modules` folders.
    pub npm: Vec<Place>,
    /// site-packages folders.
    pub pypi: Vec<Place>,
}

/// Decision 4: `CODELAW_SITE_PACKAGES`, then `$VIRTUAL_ENV`'s
/// site-packages, then `<root>/.venv`'s, without duplicates. All three are
/// searched, not only the first that is set.
pub fn python_places(project_root: &Path, env: &PackEnv) -> Vec<Place> {
    let mut out: Vec<Place> = Vec::new();
    let mut push = |dir: PathBuf, source: &'static str| {
        if !out.iter().any(|p| p.dir == dir) {
            out.push(Place { registry: Registry::Pypi, dir, source });
        }
    };
    if let Some(sp) = env.site_packages.as_deref().filter(|s| !s.is_empty()) {
        for dir in std::env::split_paths(sp).filter(|d| !d.as_os_str().is_empty()) {
            push(dir, "CODELAW_SITE_PACKAGES");
        }
    }
    if let Some(venv) = env.virtual_env.as_deref().filter(|s| !s.is_empty()) {
        for dir in venv_site_packages(Path::new(venv)) {
            push(dir, "VIRTUAL_ENV");
        }
    }
    for dir in venv_site_packages(&project_root.join(".venv")) {
        push(dir, ".venv");
    }
    out
}

impl PackSearch {
    /// The places of the project `config` governs: `[packs]` where it is
    /// set (decision 6: a set key replaces its default), the defaults
    /// elsewhere.
    pub fn new(config: &Config, env: &PackEnv) -> Self {
        let set = config.packs();
        let npm = match set.node_modules {
            Some(dirs) => dirs.into_iter().map(|dir| Place { registry: Registry::Npm, dir, source: "[packs] node_modules" }).collect(),
            None => config.config_dir().ancestors().map(|d| Place { registry: Registry::Npm, dir: d.join("node_modules"), source: "walk-up" }).collect(),
        };
        let pypi = match set.venv {
            Some(venv) => venv_site_packages(&venv).into_iter().map(|dir| Place { registry: Registry::Pypi, dir, source: "[packs] venv" }).collect(),
            None => python_places(config.config_dir(), env),
        };
        PackSearch { order: set.order.unwrap_or_else(|| vec![Registry::Npm, Registry::Pypi]), npm, pypi }
    }

    /// Every place, in the order a pack published on both registries is
    /// looked for.
    pub fn places(&self) -> Vec<&Place> {
        self.order.iter().flat_map(|r| self.registry(*r)).collect()
    }

    fn registry(&self, r: Registry) -> &[Place] {
        match r {
            Registry::Npm => &self.npm,
            Registry::Pypi => &self.pypi,
        }
    }

    /// The npm package `package`, if `npm` is in `order`.
    pub fn find_npm(&self, package: &str) -> Option<PathBuf> {
        self.find(Registry::Npm, package)
    }

    /// The PyPI distribution `distribution`, as
    /// `<site-packages>/codelaw_rules_<name>/` (decision 5), if `pypi` is in
    /// `order`.
    pub fn find_pypi(&self, distribution: &str) -> Option<PathBuf> {
        self.find(Registry::Pypi, &distribution.replace(['-', '.'], "_").to_ascii_lowercase())
    }

    fn find(&self, r: Registry, folder: &str) -> Option<PathBuf> {
        if !self.order.contains(&r) {
            return None;
        }
        self.registry(r).iter().map(|p| p.dir.join(folder)).find(|d| d.join("codelaw-pack.toml").is_file())
    }

    /// A first-party pack, on the registries it is published on, in `order`.
    pub fn find_first_party(&self, short: &str) -> Option<PathBuf> {
        let p = first_party(short);
        self.order.iter().find_map(|r| match r {
            Registry::Npm => p.npm.and_then(|n| self.find_npm(n)),
            Registry::Pypi => p.pypi.and_then(|d| self.find_pypi(d)),
        })
    }

    /// The folders searched for a pack published on `npm` and `pypi`, for
    /// the missing-pack error (decision 8).
    pub fn searched(&self, npm: bool, pypi: bool) -> String {
        let lines: Vec<String> = self
            .places()
            .into_iter()
            .filter(|p| match p.registry {
                Registry::Npm => npm,
                Registry::Pypi => pypi,
            })
            .map(|p| format!("\n  {} ({})", codelaw_core::normalize_path(&p.dir), p.source))
            .collect();
        if lines.is_empty() {
            return "\nno folder was searched: `[packs] order` leaves out every registry this pack is on".into();
        }
        format!("\nsearched:{}", lines.concat())
    }

    /// The error for a missing first-party pack, naming every install it
    /// has, and saying so when it is on one registry only.
    pub fn missing_first_party(spec: &str, short: &str) -> String {
        let p = first_party(short);
        match (p.npm, p.pypi) {
            (Some(npm), Some(pypi)) => {
                format!("`{spec}` needs the rule pack {npm} (npm) or {pypi} (PyPI); install it with `npm i -D {npm}` or `pip install {pypi}`")
            }
            (Some(npm), None) => format!("`{spec}` needs the rule pack {npm}, which is published on npm only; install it with `npm i -D {npm}`"),
            (None, Some(pypi)) => format!("`{spec}` needs the rule pack {pypi}, which is published on PyPI only; install it with `pip install {pypi}`"),
            (None, None) => unreachable!("every first-party pack is on a registry"),
        }
    }

    /// The error for a missing first-party pack: the install commands,
    /// then every folder searched.
    pub fn missing_first_party_in(&self, spec: &str, short: &str) -> String {
        let p = first_party(short);
        format!("{}{}", Self::missing_first_party(spec, short), self.searched(p.npm.is_some(), p.pypi.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("codelaw-search-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        fs::canonicalize(&d).unwrap()
    }

    /// A PyPI copy of `rules-python` in `venv`.
    fn pypi_pack(venv: &Path) -> PathBuf {
        let d = venv.join("lib/python3.12/site-packages/codelaw_rules_python");
        fs::create_dir_all(&d).unwrap();
        fs::write(d.join("codelaw-pack.toml"), "").unwrap();
        d
    }

    fn config(dir: &Path, text: &str) -> Config {
        fs::write(dir.join("codelaw.toml"), text).unwrap();
        codelaw_config::find_config(dir).unwrap()
    }

    /// DoD 1: a `VIRTUAL_ENV` without the pack no longer hides `./.venv`.
    #[test]
    fn a_foreign_virtual_env_falls_back_to_the_project_venv() {
        let d = temp("fallback");
        let project = d.join("project");
        fs::create_dir_all(&project).unwrap();
        let foreign = d.join("other/.venv");
        fs::create_dir_all(foreign.join("lib/python3.12/site-packages")).unwrap();
        let want = pypi_pack(&project.join(".venv"));
        let env = PackEnv { site_packages: None, virtual_env: Some(foreign.to_string_lossy().into_owned()) };
        let s = PackSearch::new(&config(&project, ""), &env);
        assert_eq!(s.find_pypi("codelaw-rules-python"), Some(want));
        let sources: Vec<&str> = s.pypi.iter().map(|p| p.source).collect();
        assert_eq!(sources, ["VIRTUAL_ENV", ".venv"]);
        // A pack in both still loads from `$VIRTUAL_ENV`.
        let theirs = pypi_pack(&foreign);
        assert_eq!(PackSearch::new(&config(&project, ""), &env).find_pypi("codelaw-rules-python"), Some(theirs));
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn the_python_places_join_and_drop_duplicates() {
        let d = temp("join");
        let sp = d.join(".venv/lib/python3.12/site-packages");
        fs::create_dir_all(&sp).unwrap();
        let launcher = std::env::join_paths([d.join("launcher"), sp.clone()]).unwrap().into_string().unwrap();
        let env = PackEnv { site_packages: Some(launcher), virtual_env: Some(d.join(".venv").to_string_lossy().into_owned()) };
        let got: Vec<(PathBuf, &str)> = python_places(&d, &env).into_iter().map(|p| (p.dir, p.source)).collect();
        assert_eq!(got, [(d.join("launcher"), "CODELAW_SITE_PACKAGES"), (sp, "CODELAW_SITE_PACKAGES")]);
        assert!(python_places(&d, &PackEnv { site_packages: Some(String::new()), virtual_env: Some(String::new()) }).iter().all(|p| p.source == ".venv"));
        let _ = fs::remove_dir_all(&d);
    }

    /// DoD 3: a set key replaces its default.
    #[test]
    fn a_set_key_replaces_its_default() {
        let d = temp("replace");
        let theirs = d.join("theirs");
        pypi_pack(&theirs);
        let env = PackEnv { site_packages: None, virtual_env: Some(theirs.to_string_lossy().into_owned()) };
        // `venv` set: a `VIRTUAL_ENV` holding the pack is ignored.
        fs::create_dir_all(d.join("pinned/lib/python3.12/site-packages")).unwrap();
        let s = PackSearch::new(&config(&d, "[packs]\nvenv = \"pinned\"\n"), &env);
        assert_eq!(s.find_pypi("codelaw-rules-python"), None);
        assert!(s.pypi.iter().all(|p| p.source == "[packs] venv"));
        // `node_modules` set: no walk-up.
        let up = d.join("node_modules/@codelaw/rules-js");
        fs::create_dir_all(&up).unwrap();
        fs::write(up.join("codelaw-pack.toml"), "").unwrap();
        let project = d.join("app");
        fs::create_dir_all(project.join("vendor")).unwrap();
        assert_eq!(PackSearch::new(&config(&project, ""), &env).find_npm("@codelaw/rules-js"), Some(up.clone()));
        let s = PackSearch::new(&config(&project, "root = true\n[packs]\nnode_modules = [\"vendor\"]\n"), &env);
        assert_eq!(s.find_npm("@codelaw/rules-js"), None);
        assert_eq!(s.npm.len(), 1);
        // `order = ["pypi"]` never reads node_modules.
        let s = PackSearch::new(&config(&project, "root = true\n[packs]\norder = [\"pypi\"]\n"), &env);
        assert_eq!(s.find_npm("@codelaw/rules-js"), None);
        assert_eq!(s.find_first_party("js"), None);
        assert!(s.places().iter().all(|p| p.registry == Registry::Pypi));
        let m = s.missing_first_party_in("codelaw:js/recommended", "js");
        assert!(!m.contains("walk-up") && !m.contains("node_modules ("), "{m}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn the_missing_pack_error_lists_every_folder_searched() {
        let d = temp("missing");
        let s = PackSearch::new(&config(&d, ""), &PackEnv::default());
        let m = s.missing_first_party_in("codelaw:js/recommended", "js");
        assert!(m.starts_with("`codelaw:js/recommended` needs the rule pack @codelaw/rules-js"), "{m}");
        assert!(m.contains(&format!("{} (walk-up)", codelaw_core::normalize_path(&d.join("node_modules")))), "{m}");
    }
}
