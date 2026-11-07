---
type: Handoff
name: W-173
description: "Dependabot: five open advisories (1 critical, 3 high, 1 moderate) on `zip` 8.6.0 in Cargo.lock — transitive, through ruff_db ← ruff_python_formatter ← codelaw-format; fixed in zip 9.0.5."
item: W-173
filed: 2026-11-07
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-07
updated_by: "Claude Code (session 0a172e1f)"
ball: arpit
---

# W-173 — `zip` 8.6.0's five advisories (Dependabot alerts 1–5)

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Find whether any codelaw path reaches `zip` (ruff_db's vendored-typeshed or archive code; the formatter should not) | the real exposure | ✅ none reachable |
| 2 | Drop it if a `ruff_db` feature or a narrower ruff crate removes it (removing a dependency needs no ratification); else a ruff release on `zip` ≥ 9.0.5, or a `[patch]` — any new version or crate goes to the decider under L11 | the advisories closed | 🔴 removal impossible; bump to Ruff `=0.0.21` is the decider's (inbox) |
| 3 | `cargo test`, the formatter corpus unchanged, Dependabot alerts closed | proof | ⏳ after the ruling |

## What is known

- Alerts (2026-11-07): #1 critical — parser differential / security-control bypass (fixed 9.0.5); #2, #4, #5 high — unbounded symlink target, file write outside the extraction directory, legacy-decoder DoS (fixed 9.0.0); #3 moderate — AES HMAC bypass on premature EOF (fixed 9.0.0).
- `cargo tree -i zip`: `zip 8.6.0 ← ruff_db 0.0.18 ← ruff_python_formatter 0.0.18 ← codelaw-format`. SR-LAW-11's ledger lists `zip@8`.

## Record

[SR-LAW-11](../../records/0013_LAW-11-dependencies.md) (the ledger); [SR-FORMATTER](../../records/0105_formatter.md) if the Python formatter's crates change.

## Step 1 — exposure: none reachable (2026-11-07)

- codelaw calls one Ruff entry point: `ruff_python_formatter::format_module_source(text, options)` (`crates/codelaw-format/src/engines.rs`, `fn ruff`). It parses and prints a string; it never touches `ruff_db`.
- `zip` is used in `ruff_db` only by `src/vendored.rs` — `VendoredFileSystem::new(bytes)`, ty's zipped typeshed — and that is reached only through a `ruff_db::Db` implementation (`formatted_file`). codelaw implements no `Db` and builds no `VendoredFileSystem`; no `crates/` file names `ruff_db`.
- The five advisories are about reading or extracting a hostile archive. codelaw never hands `zip` any bytes. Some `zip` reader code is still linked into the release binary (`nm`: the `ZipError` impls, `ZipFile`/`CryptoReader` over `ruff_db::vendored::ArchiveData`), kept by `ruff_db`'s generic code, but nothing calls it.

## Step 2 — removal is impossible; the fix is a version change (the decider's)

- **No feature drops it.** `ruff_db 0.0.18` takes `zip ^8.0` as a non-optional dependency (`default-features = false`, no feature gate), and `ruff_python_formatter 0.0.18` takes `ruff_db` as non-optional. No narrower ruff crate gives the formatter without `ruff_db`.
- **The fix exists upstream.** crates.io, 2026-11-07: `ruff_db` 0.0.19 and 0.0.20 still take `zip ^8.0`; **`ruff_db 0.0.21` (2026-11-05) takes `zip ^9.0.5`**, and `ruff_python_formatter 0.0.21` takes `ruff_db ^0.0.21`.
- **What the bump changes** (a scratch crate with D-02's five crates at `=0.0.18` and at `=0.0.21`, `cargo tree -e normal`): every `ruff_*` crate and `ty_static` 0.0.18 → 0.0.21 (sixteen on normal edges); `zip` 8.6.0 → 9.4.1 (MIT); `typed-path` 0.12.4 → 0.13.0 (MIT OR Apache-2.0); `phf`/`phf_shared` 0.11.3 → 0.14.0 (already in the lock); **new:** `getrandom` 0.4.3 (MIT OR Apache-2.0, all targets) and `nix` 0.31.3 (MIT, macOS host); **gone:** `psm`, `stacker`. All permissive (L2).
- **A `[patch]`** of `ruff_db` 0.0.18 onto `zip` 9 needs a fork (zip 9's API differs) — rejected.
- Filed to the inbox, 2026-11-07, options recommendation first.
