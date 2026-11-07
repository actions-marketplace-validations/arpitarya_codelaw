---
type: Handoff
name: W-173
description: "Dependabot: five open advisories (1 critical, 3 high, 1 moderate) on `zip` 8.6.0 in Cargo.lock — transitive, through ruff_db ← ruff_python_formatter ← codelaw-format; fixed in zip 9.0.5."
item: W-173
filed: 2026-11-07
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-07
updated_by: "Claude Code (session 0a172e1f)"
ball: agent
---

# W-173 — `zip` 8.6.0's five advisories (Dependabot alerts 1–5)

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Find whether any codelaw path reaches `zip` (ruff_db's vendored-typeshed or archive code; the formatter should not) | the real exposure | ⏳ |
| 2 | Drop it if a `ruff_db` feature or a narrower ruff crate removes it (removing a dependency needs no ratification); else a ruff release on `zip` ≥ 9.0.5, or a `[patch]` — any new version or crate goes to the decider under L11 | the advisories closed | ⏳ |
| 3 | `cargo test`, the formatter corpus unchanged, Dependabot alerts closed | proof | ⏳ |

## What is known

- Alerts (2026-11-07): #1 critical — parser differential / security-control bypass (fixed 9.0.5); #2, #4, #5 high — unbounded symlink target, file write outside the extraction directory, legacy-decoder DoS (fixed 9.0.0); #3 moderate — AES HMAC bypass on premature EOF (fixed 9.0.0).
- `cargo tree -i zip`: `zip 8.6.0 ← ruff_db 0.0.18 ← ruff_python_formatter 0.0.18 ← codelaw-format`. SR-LAW-11's ledger lists `zip@8`.

## Record

[SR-LAW-11](../../records/0013_LAW-11-dependencies.md) (the ledger); [SR-FORMATTER](../../records/0105_formatter.md) if the Python formatter's crates change.
