//! `[packs]`: where the engine finds a rule pack (SR-PACK-LOOKUP decisions
//! 5–7). The loader parses and checks the section here; the lookup itself is
//! `codelaw_rules::search`, which reads [`Config::packs`].

use crate::ConfigError;
use crate::resolve::Config;
use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::{Path, PathBuf};

/// `[packs]` as written. Every key that is set replaces its default
/// (decision 6).
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawPacks {
    /// The registries searched, in order: `npm`, `pypi`, or both.
    #[serde(default, deserialize_with = "order")]
    pub order: Option<Vec<String>>,
    /// The folders that hold npm packages, in order; no walk-up.
    #[serde(default, deserialize_with = "node_modules")]
    pub node_modules: Option<Vec<String>>,
    /// One virtual environment; its site-packages are the only Python place.
    #[serde(default, deserialize_with = "venv")]
    pub venv: Option<String>,
}

/// A wrong type names its key (decision 7), not only its line.
fn typed<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D, key: &str, want: &str) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some).map_err(|_| serde::de::Error::custom(format!("`[packs] {key}` must be {want}")))
}

fn order<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<String>>, D::Error> {
    typed(d, "order", "an array of `\"npm\"` and `\"pypi\"`")
}

fn node_modules<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<String>>, D::Error> {
    typed(d, "node_modules", "an array of folders")
}

fn venv<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    typed(d, "venv", "one folder, as a string")
}

/// The values of `[packs] order`.
pub const PACK_REGISTRIES: &[&str] = &["npm", "pypi"];

/// A registry a pack is looked up on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Registry {
    Npm,
    Pypi,
}

impl Registry {
    pub fn name(self) -> &'static str {
        match self {
            Registry::Npm => "npm",
            Registry::Pypi => "pypi",
        }
    }
}

/// `[packs]` resolved against the root config's folder; `None` is the
/// default for that key.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PacksSetting {
    pub order: Option<Vec<Registry>>,
    pub node_modules: Option<Vec<PathBuf>>,
    pub venv: Option<PathBuf>,
}

/// A venv's site-packages folders (decision 5): every
/// `lib/python*/site-packages` that exists, sorted by name, then
/// `Lib/site-packages` (Windows).
pub fn venv_site_packages(venv: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(venv.join("lib")) {
        let mut names: Vec<PathBuf> = entries
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().starts_with("python"))
            .map(|e| e.path().join("site-packages"))
            .filter(|p| p.is_dir())
            .collect();
        names.sort();
        out.extend(names);
    }
    let windows = venv.join("Lib").join("site-packages");
    if windows.is_dir() {
        out.push(windows);
    }
    out
}

/// Decision 7: each of these is a load error naming the key.
pub(crate) fn check(raw: &RawPacks, config: &Path, dir: &Path) -> Result<(), ConfigError> {
    let err = |m: String| Err(ConfigError::new(Some(config.to_path_buf()), m));
    if let Some(order) = &raw.order {
        if order.is_empty() {
            return err("`[packs] order` is empty; name `npm`, `pypi` or both".into());
        }
        for (i, r) in order.iter().enumerate() {
            if !PACK_REGISTRIES.contains(&r.as_str()) {
                return err(format!("`[packs] order`: `{r}` is not one of `npm`, `pypi`"));
            }
            if order[..i].contains(r) {
                return err(format!("`[packs] order`: `{r}` is named twice"));
            }
        }
    }
    for f in raw.node_modules.iter().flatten() {
        let p = dir.join(f);
        if !p.is_dir() {
            return err(format!("`[packs] node_modules`: {} is not a folder", codelaw_core::normalize_path(&p)));
        }
    }
    if let Some(v) = &raw.venv {
        let p = dir.join(v);
        if !p.is_dir() {
            return err(format!("`[packs] venv`: {} is not a folder", codelaw_core::normalize_path(&p)));
        }
        if venv_site_packages(&p).is_empty() {
            return err(format!("`[packs] venv`: {} has no lib/python*/site-packages or Lib/site-packages folder", codelaw_core::normalize_path(&p)));
        }
    }
    Ok(())
}

impl Config {
    /// `[packs]` of the root config, its folders made absolute
    /// (decision 6: relative to the root config's folder).
    pub fn packs(&self) -> PacksSetting {
        let Some(raw) = self.root_raw().map(|r| &r.packs) else { return PacksSetting::default() };
        let dir = self.config_dir();
        PacksSetting {
            order: raw.order.as_ref().map(|o| o.iter().map(|r| if r == "pypi" { Registry::Pypi } else { Registry::Npm }).collect()),
            node_modules: raw.node_modules.as_ref().map(|n| n.iter().map(|f| dir.join(f)).collect()),
            venv: raw.venv.as_ref().map(|v| dir.join(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("codelaw-packs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn load(dir: &Path, text: &str) -> Result<Config, ConfigError> {
        Config::from_toml(dir.to_path_buf(), text)
    }

    #[test]
    fn packs_resolves_against_the_config_folder() {
        let d = temp("resolve");
        fs::create_dir_all(d.join("vendor/node_modules")).unwrap();
        fs::create_dir_all(d.join("env/lib/python3.12/site-packages")).unwrap();
        let c = load(&d, "[packs]\norder = [\"pypi\"]\nnode_modules = [\"vendor/node_modules\"]\nvenv = \"env\"\n").unwrap();
        let p = c.packs();
        assert_eq!(p.order, Some(vec![Registry::Pypi]));
        assert_eq!(p.node_modules, Some(vec![d.join("vendor/node_modules")]));
        assert_eq!(p.venv, Some(d.join("env")));
        assert_eq!(load(&d, "").unwrap().packs(), PacksSetting::default());
        let _ = fs::remove_dir_all(&d);
    }

    /// Decision 7: one test per load error.
    #[test]
    fn every_packs_load_error_names_its_key() {
        let d = temp("errors");
        fs::create_dir_all(d.join("empty-venv")).unwrap();
        for (text, want) in [
            ("[packs]\nordr = []\n", "unknown key `ordr`"),
            ("[packs]\norder = \"npm\"\n", "`[packs] order` must be an array"),
            ("[packs]\nvenv = [\".venv\"]\n", "`[packs] venv` must be one folder"),
            ("[packs]\norder = []\n", "`[packs] order` is empty"),
            ("[packs]\norder = [\"cargo\"]\n", "`[packs] order`: `cargo` is not one of"),
            ("[packs]\norder = [\"npm\", \"npm\"]\n", "`[packs] order`: `npm` is named twice"),
            ("[packs]\nnode_modules = [\"nowhere\"]\n", "`[packs] node_modules`: "),
            ("[packs]\nvenv = \"nowhere\"\n", "`[packs] venv`: "),
            ("[packs]\nvenv = \"empty-venv\"\n", "has no lib/python*/site-packages"),
        ] {
            let e = load(&d, text).expect_err(text).to_string();
            assert!(e.contains(want), "{text}: {e}");
        }
        let _ = fs::remove_dir_all(&d);
    }

    /// Decision 7: `[packs]` is root-only, and a profile has no such key.
    #[test]
    fn packs_is_root_only() {
        let d = temp("nested");
        fs::write(d.join("codelaw.toml"), "").unwrap();
        fs::create_dir_all(d.join("pkg/node_modules")).unwrap();
        fs::write(d.join("pkg/codelaw.toml"), "[packs]\nnode_modules = [\"node_modules\"]\n").unwrap();
        let e = crate::find_config(&d.join("pkg")).unwrap_err().to_string();
        assert!(e.contains("[packs] can only be set in the root config"), "{e}");
        assert!(load(&d, "[profile.ci.packs]\norder = [\"npm\"]\n").is_err(), "a profile may not set [packs]");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn site_packages_are_sorted_then_windows() {
        let d = temp("site");
        for p in ["lib/python3.12/site-packages", "lib/python3.11/site-packages", "lib/pythonx", "Lib/site-packages"] {
            fs::create_dir_all(d.join(p)).unwrap();
        }
        let got = venv_site_packages(&d);
        // On a case-insensitive filesystem `lib` and `Lib` are one folder.
        assert_eq!(&got[..2], &[d.join("lib/python3.11/site-packages"), d.join("lib/python3.12/site-packages")]);
        assert_eq!(got.last(), Some(&d.join("Lib/site-packages")));
        let _ = fs::remove_dir_all(&d);
    }
}
