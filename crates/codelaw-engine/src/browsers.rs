//! codelaw's own browser-support table, the data package
//! `@codelaw/data-browsers` / `codelaw-data-browsers` (L6 decision 15;
//! SR-RULE-FORMAT decision 2c; W-166): per feature id, the first version of
//! each browser that supports it unprefixed. Optional: without it, a
//! browser-gated fix keeps its `suggest` rung.
//!
//! Found where packs are found (SR-PACK-LOOKUP): `node_modules/@codelaw/
//! data-browsers/`, or `<site-packages>/codelaw_data_browsers/`, holding
//! `data/browsers.json`. Read once per run, from disk only (L5).

use crate::Project;
use codelaw_config::Registry;
use codelaw_rules::PackSearch;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The table's file inside the package.
pub const TABLE_FILE: &str = "data/browsers.json";
/// The npm package and the PyPI distribution.
pub const NPM_NAME: &str = "@codelaw/data-browsers";
pub const PYPI_NAME: &str = "codelaw-data-browsers";
/// The table schemas this engine reads.
pub const SCHEMAS: &[u64] = &[1];

/// The table as loaded.
#[derive(Clone, Debug, Default)]
pub struct BrowserTable {
    /// The data's own version, `YYYY.M.D` (L6 decision 15).
    pub version: String,
    /// The folder it was loaded from.
    pub dir: PathBuf,
    features: BTreeMap<String, BTreeMap<String, Vec<u64>>>,
}

impl BrowserTable {
    /// Each browser's first unprefixed version of `feature`.
    pub fn feature(&self, feature: &str) -> Option<&BTreeMap<String, Vec<u64>>> {
        self.features.get(feature)
    }

    /// Parse `data/browsers.json`; `Err` says what is wrong.
    pub fn parse(text: &str, dir: &Path) -> Result<BrowserTable, String> {
        let v: serde_json::Value = serde_json::from_str(text).map_err(|e| format!("not JSON: {e}"))?;
        let schema = v.get("schema").and_then(|s| s.as_u64()).ok_or("no `schema`")?;
        if !SCHEMAS.contains(&schema) {
            return Err(format!("schema {schema} is newer than this codelaw reads; upgrade codelaw"));
        }
        let version = v.get("version").and_then(|s| s.as_str()).ok_or("no `version`")?.to_owned();
        let mut features = BTreeMap::new();
        for (id, row) in v.get("features").and_then(|f| f.as_object()).ok_or("no `features`")? {
            let since = row.get("since").and_then(|s| s.as_object()).ok_or_else(|| format!("`{id}` has no `since`"))?;
            let mut out = BTreeMap::new();
            for (browser, ver) in since {
                if !codelaw_config::TARGET_BROWSERS.contains(&browser.as_str()) {
                    return Err(format!("`{id}` names `{browser}`, not one of {}", codelaw_config::TARGET_BROWSERS.join(", ")));
                }
                let parsed: Option<Vec<u64>> = ver.as_str().and_then(|s| s.split('.').map(|p| p.parse().ok()).collect());
                out.insert(browser.clone(), parsed.ok_or_else(|| format!("`{id}`: `{browser}` is not a version"))?);
            }
            features.insert(id.clone(), out);
        }
        Ok(BrowserTable { version, dir: dir.to_owned(), features })
    }
}

/// The package's folder, by the search packs use, in its order.
pub fn find(search: &PackSearch) -> Option<PathBuf> {
    search.order.iter().find_map(|r| {
        let (places, folder) = match r {
            Registry::Npm => (&search.npm, NPM_NAME.to_owned()),
            Registry::Pypi => (&search.pypi, PYPI_NAME.replace('-', "_")),
        };
        places.iter().map(|p| p.dir.join(&folder)).find(|d| d.join(TABLE_FILE).is_file())
    })
}

impl Project {
    /// The browser-support table, when its package is installed; a table
    /// that cannot be read is a warning and no table.
    pub fn browser_table(&self) -> Option<&BrowserTable> {
        self.browsers
            .get_or_init(|| {
                let dir = find(&crate::pack_search(&self.config, &self.options))?;
                let text = std::fs::read_to_string(dir.join(TABLE_FILE)).ok()?;
                match BrowserTable::parse(&text, &dir) {
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
        let t = BrowserTable::parse(r#"{"schema":1,"version":"2026.11.5","features":{"css-property:user-select":{"since":{"chrome":"54","safari":"16.4"},"sources":["x"]}}}"#, Path::new(".")).unwrap();
        assert_eq!(t.feature("css-property:user-select").unwrap()["safari"], vec![16, 4]);
        assert!(BrowserTable::parse(r#"{"schema":9,"version":"x","features":{}}"#, Path::new(".")).unwrap_err().contains("newer"));
        assert!(BrowserTable::parse(r#"{"schema":1,"version":"x","features":{"a":{"since":{"opera":"1"}}}}"#, Path::new(".")).unwrap_err().contains("opera"));
    }
}
