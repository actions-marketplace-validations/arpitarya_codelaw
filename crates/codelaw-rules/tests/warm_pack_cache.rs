//! A process that loads a project per request (`mcp`, `lsp`, the daemon)
//! keeps each pack from its first load: the second request must not parse
//! every pack again (CI-mcp-p95). Its own test binary, because the switch is
//! process-wide.

use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn a_warm_process_keeps_a_pack_from_its_first_load() {
    let pack: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "packs", "airflow"].iter().collect();
    codelaw_rules::keep_packs_from_first_load();
    let first = codelaw_rules::load_pack_cached(&pack, true).expect("the airflow pack loads");
    let second = codelaw_rules::load_pack_cached(&pack, true).expect("and loads again");
    assert!(Arc::ptr_eq(&first, &second), "the second load of a pack parsed it again");
}
