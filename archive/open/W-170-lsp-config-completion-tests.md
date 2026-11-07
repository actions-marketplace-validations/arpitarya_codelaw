---
type: Handoff
name: W-170
description: "Three `codelaw-lsp` config-completion tests fail on main (reported 2026-11-05 by W-166's build, failing before its changes too) — CI's `test` job goes red on them."
item: W-170
filed: 2026-11-05
filed_by: "Claude Code (session 0a172e1f)"
updated: 2026-11-05
updated_by: "Claude Code (session 0a172e1f)"
ball: agent
---

# W-170 — `codelaw-lsp`'s config-completion tests fail on main

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Run `cargo test -p codelaw-lsp`; name the three tests and the commit that broke them (`git bisect` over today's landings: W-158's `reason`/`require_reason`, W-166's `[fix]` and `[targets]` keys, the schema) | the cause, not a guess | ✅ `c0346694` (W-158) |
| 2 | Fix the cause — completion offering the new keys, or the test's expected list if a ruling changed it | a green `test` job | ✅ |

## What is known

W-166's agent found them failing against the pre-change schema too, so they predate W-166's own commits; W-157 built config completion (keys, rule ids, presets, severities from the installed packs), and W-158 and W-166 added config keys the same day.

## Outcome (2026-11-05)

The three: `completions_come_from_the_schema_and_the_packs`, `fix_offers_only_the_rule_s_own_rung_and_below`, `the_server_publishes_completes_and_fixes_a_config_as_typed`. Cause: W-158 (`c0346694`) gave the top-level `[rules]` its own schema node, `rootRules` (rule ids plus `require_reason`); the server recognised a rules table only as `/definitions/rules`, so in `[rules]` it offered schema keys, not rule ids, and `fix` all three rungs. Fix, in `crates/codelaw-lsp/src/config.rs`: `is_rules` takes both nodes; a rules table's own key (`require_reason`) is never a rule, and is offered as a key with true/false. No test's assertion changed; three were added for `require_reason`. `cargo test -p codelaw-lsp`: 35 + 8 passed.

## Record

[SR-EDITOR](../../records/0114_editor.md) decisions 1–3 (the language server; config completion, W-157).
