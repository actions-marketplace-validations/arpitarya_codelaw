//! codelaw's polyfill table, the data package `@codelaw/data-polyfills` /
//! `codelaw-data-polyfills` (L6 decision 15; L11 D-53; W-175): per core-js
//! feature, the first version of each engine that has it built in, and which
//! core-js entry stands for which features. Optional: without it
//! `js/no-unnecessary-polyfills` reports nothing.
//!
//! Found where packs are found (SR-PACK-LOOKUP): `node_modules/@codelaw/
//! data-polyfills/`, or `<site-packages>/codelaw_data_polyfills/`, holding
//! `data/polyfills.json`; in a checkout of this repository also
//! `data-packages/polyfills/` (under the project root, or `EngineOptions::
//! data_packages`, which the rule tester sets). Read from disk only (L5), once per run, and
//! only when a rule asks for it (`FileContext::data_table`): a run that does
//! not turn the rule on never opens the file.

use crate::Project;
use codelaw_config::Registry;
use codelaw_rules::PackSearch;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The table's file inside the package.
pub const TABLE_FILE: &str = "data/polyfills.json";
/// The npm package and the PyPI distribution.
pub const NPM_NAME: &str = "@codelaw/data-polyfills";
pub const PYPI_NAME: &str = "codelaw-data-polyfills";
/// The table schemas this engine reads.
pub const SCHEMAS: &[u64] = &[1];

/// The table as loaded.
#[derive(Clone, Debug)]
pub struct PolyfillTable {
    /// The data's own version, `YYYY.M.D` (L6 decision 15).
    pub version: String,
    /// The folder it was loaded from.
    pub dir: PathBuf,
    /// The whole file, parsed once and shared with the rules that read it.
    pub value: Arc<serde_json::Value>,
}

impl PolyfillTable {
    /// Parse `data/polyfills.json`; `Err` says what is wrong.
    pub fn parse(text: &str, dir: &Path) -> Result<PolyfillTable, String> {
        let v: serde_json::Value = serde_json::from_str(text).map_err(|e| format!("not JSON: {e}"))?;
        let schema = v.get("schema").and_then(|s| s.as_u64()).ok_or("no `schema`")?;
        if !SCHEMAS.contains(&schema) {
            return Err(format!("schema {schema} is newer than this codelaw reads; upgrade codelaw"));
        }
        let version = v.get("version").and_then(|s| s.as_str()).ok_or("no `version`")?.to_owned();
        for key in ["features", "since", "entries"] {
            if v.get(key).is_none() {
                return Err(format!("no `{key}`"));
            }
        }
        Ok(PolyfillTable { version, dir: dir.to_owned(), value: Arc::new(v) })
    }
}

/// The package's folder: by the search packs use, in its order, then the
/// repository's own `data-packages/polyfills/`.
pub fn find(search: &PackSearch, root: &Path, extra: Option<&Path>) -> Option<PathBuf> {
    let installed = search.order.iter().find_map(|r| {
        let (places, folder) = match r {
            Registry::Npm => (&search.npm, NPM_NAME.to_owned()),
            Registry::Pypi => (&search.pypi, PYPI_NAME.replace('-', "_")),
        };
        places.iter().map(|p| p.dir.join(&folder)).find(|d| d.join(TABLE_FILE).is_file())
    });
    installed.or_else(|| [root.join("data-packages/polyfills"), extra.map(|e| e.join("polyfills")).unwrap_or_default()].into_iter().find(|d| d.join(TABLE_FILE).is_file()))
}

impl Project {
    /// The polyfill table, when its package is installed; a table that
    /// cannot be read is a warning and no table.
    pub fn polyfill_table(&self) -> Option<&PolyfillTable> {
        self.polyfills
            .get_or_init(|| {
                let dir = find(&crate::pack_search(&self.config, &self.options), self.root(), self.options.data_packages.as_deref())?;
                let text = std::fs::read_to_string(dir.join(TABLE_FILE)).ok()?;
                match PolyfillTable::parse(&text, &dir) {
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
        let ok = r#"{"schema":1,"version":"2026.11.8","engines":["chrome"],"features":["es.array.flat"],"since":[{"chrome":"69"}],"entries":{"core-js/es/array/flat":[0]}}"#;
        assert_eq!(PolyfillTable::parse(ok, Path::new(".")).unwrap().version, "2026.11.8");
        assert!(PolyfillTable::parse(r#"{"schema":9,"version":"x"}"#, Path::new(".")).unwrap_err().contains("newer"));
        assert!(PolyfillTable::parse(r#"{"schema":1,"version":"x"}"#, Path::new(".")).unwrap_err().contains("features"));
    }
}
