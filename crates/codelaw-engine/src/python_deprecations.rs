//! codelaw's table of deprecated Python standard-library names, the data
//! package `@codelaw/data-python-deprecations` /
//! `codelaw-data-python-deprecations` (L6 decision 15; L11 D-54; W-187): per
//! module, class, function or method, attribute, decorator and argument, the
//! Python version that deprecates it and the one that removes it. Optional:
//! without it `py/deprecated-module`, `-class`, `-method`, `-attribute` and
//! `-argument` report nothing.
//!
//! Found where packs are found (SR-PACK-LOOKUP): `node_modules/@codelaw/
//! data-python-deprecations/`, or `<site-packages>/codelaw_data_python_
//! deprecations/`, holding `data/python-deprecations.json`; in a checkout of
//! this repository also `data-packages/python-deprecations/` (under the
//! project root, or `EngineOptions::data_packages`, which the rule tester
//! sets). Read from disk only (L5), once per run, and only when a rule asks
//! for it (`FileContext::data_table`): a run that does not turn the rules on
//! never opens the file.

use crate::Project;
use codelaw_config::Registry;
use codelaw_rules::PackSearch;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The table's file inside the package.
pub const TABLE_FILE: &str = "data/python-deprecations.json";
/// The npm package and the PyPI distribution.
pub const NPM_NAME: &str = "@codelaw/data-python-deprecations";
pub const PYPI_NAME: &str = "codelaw-data-python-deprecations";
/// The folder name in a checkout's `data-packages/`.
pub const SHORT: &str = "python-deprecations";
/// The table schemas this engine reads.
pub const SCHEMAS: &[u64] = &[1];

/// The table as loaded.
#[derive(Clone, Debug)]
pub struct PythonDeprecationTable {
    /// The data's own version, `YYYY.M.D` (L6 decision 15).
    pub version: String,
    /// The folder it was loaded from.
    pub dir: PathBuf,
    /// The whole file, parsed once and shared with the rules that read it.
    pub value: Arc<serde_json::Value>,
}

impl PythonDeprecationTable {
    /// Parse `data/python-deprecations.json`; `Err` says what is wrong.
    pub fn parse(text: &str, dir: &Path) -> Result<PythonDeprecationTable, String> {
        let v: serde_json::Value = serde_json::from_str(text).map_err(|e| format!("not JSON: {e}"))?;
        let schema = v.get("schema").and_then(|s| s.as_u64()).ok_or("no `schema`")?;
        if !SCHEMAS.contains(&schema) {
            return Err(format!("schema {schema} is newer than this codelaw reads; upgrade codelaw"));
        }
        let version = v.get("version").and_then(|s| s.as_str()).ok_or("no `version`")?.to_owned();
        for key in ["modules", "classes", "functions", "attributes", "decorators", "arguments"] {
            if v.get(key).and_then(|t| t.as_object()).is_none() {
                return Err(format!("no `{key}`"));
            }
        }
        Ok(PythonDeprecationTable { version, dir: dir.to_owned(), value: Arc::new(v) })
    }
}

/// The package's folder: by the search packs use, in its order, then the
/// repository's own `data-packages/python-deprecations/`.
pub fn find(search: &PackSearch, root: &Path, extra: Option<&Path>) -> Option<PathBuf> {
    let installed = search.order.iter().find_map(|r| {
        let (places, folder) = match r {
            Registry::Npm => (&search.npm, NPM_NAME.to_owned()),
            Registry::Pypi => (&search.pypi, PYPI_NAME.replace('-', "_")),
        };
        places.iter().map(|p| p.dir.join(&folder)).find(|d| d.join(TABLE_FILE).is_file())
    });
    installed.or_else(|| [root.join("data-packages").join(SHORT), extra.map(|e| e.join(SHORT)).unwrap_or_default()].into_iter().find(|d| d.join(TABLE_FILE).is_file()))
}

impl Project {
    /// The deprecation table, when its package is installed; a table that
    /// cannot be read is a warning and no table.
    pub fn python_deprecation_table(&self) -> Option<&PythonDeprecationTable> {
        self.python_deprecations
            .get_or_init(|| {
                let dir = find(&crate::pack_search(&self.config, &self.options), self.root(), self.options.data_packages.as_deref())?;
                let text = std::fs::read_to_string(dir.join(TABLE_FILE)).ok()?;
                match PythonDeprecationTable::parse(&text, &dir) {
                    Ok(t) => Some(t),
                    Err(why) => {
                        eprintln!("codelaw: warning: {} is not read: {why}", codelaw_core::normalize_path(&dir.join(TABLE_FILE)));
                        None
                    }
                }
            })
            .as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_table_parses_and_refuses_what_it_cannot_read() {
        let ok = r#"{"schema":1,"version":"2026.11.16","modules":{"asyncore":["3.6","3.12"]},"classes":{},"functions":{},"attributes":{},"decorators":{},"arguments":{}}"#;
        assert_eq!(PythonDeprecationTable::parse(ok, Path::new(".")).unwrap().version, "2026.11.16");
        assert!(PythonDeprecationTable::parse(r#"{"schema":9,"version":"x"}"#, Path::new(".")).unwrap_err().contains("newer"));
        assert!(PythonDeprecationTable::parse(r#"{"schema":1,"version":"x"}"#, Path::new(".")).unwrap_err().contains("modules"));
    }
}
