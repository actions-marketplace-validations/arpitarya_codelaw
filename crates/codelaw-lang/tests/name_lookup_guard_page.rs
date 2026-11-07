//! W-169: `codelaw rules lint` and `codelaw test` died with SIGSEGV now and
//! then on the musl build legs. The core (CI run 42348986474,
//! linux-arm64-musl) showed tree-sitter's `ts_language_symbol_for_name` →
//! `strncmp(symbol, name, len)` reading `name[len]`, one past a 16-byte kind
//! name whose allocation ended at the end of a page.
//!
//! This puts a name flush against an unreadable page and looks it up: the
//! engine must not read past it. Before the fix it faults on the musl
//! binaries; on a libc whose `strncmp` stays within `len` it passes either way.
#![cfg(unix)]

use codelaw_lang::{AgLanguage, Grammar};

/// Two pages, the second unreadable, and `name` written so it ends exactly
/// where the first page ends.
struct GuardedName {
    base: *mut libc::c_void,
    page: usize,
    len: usize,
}

impl GuardedName {
    fn new(name: &str) -> GuardedName {
        // SAFETY: plain mmap/mprotect of a fresh anonymous mapping; every
        // pointer written stays inside its first page.
        unsafe {
            let page = usize::try_from(libc::sysconf(libc::_SC_PAGESIZE)).expect("a page size");
            let base = libc::mmap(std::ptr::null_mut(), 2 * page, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
            assert_ne!(base, libc::MAP_FAILED, "mmap");
            assert_eq!(libc::mprotect(base.cast::<u8>().add(page).cast(), page, libc::PROT_NONE), 0, "mprotect");
            let start = base.cast::<u8>().add(page - name.len());
            std::ptr::copy_nonoverlapping(name.as_ptr(), start, name.len());
            GuardedName { base, page, len: name.len() }
        }
    }

    fn get(&self) -> &str {
        // SAFETY: the bytes were copied from a `str` and the mapping lives as
        // long as `self`.
        unsafe {
            let start = self.base.cast::<u8>().add(self.page - self.len);
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(start, self.len))
        }
    }
}

impl Drop for GuardedName {
    fn drop(&mut self) {
        // SAFETY: the mapping `new` made, unmapped once.
        unsafe {
            libc::munmap(self.base, 2 * self.page);
        }
    }
}

#[test]
fn a_kind_or_field_name_at_the_end_of_a_page_is_not_read_past() {
    let grammars = [Grammar::JavaScript, Grammar::TypeScript, Grammar::Tsx, Grammar::Css, Grammar::Html, Grammar::Python];
    // Kinds and fields that exist (strncmp agrees for all `len` bytes, the
    // case that read past) and a prefix of one (`identifie`).
    for name in ["identifier", "call_expression", "template_string", "identifie", "body", "arguments"] {
        let guarded = GuardedName::new(name);
        for g in grammars {
            let _ = g.kind_to_id(guarded.get());
            let _ = g.field_to_id(guarded.get());
        }
    }
    let guarded = GuardedName::new("call_expression");
    assert_ne!(Grammar::JavaScript.kind_to_id(guarded.get()), 0, "the lookup still finds the kind");
}
