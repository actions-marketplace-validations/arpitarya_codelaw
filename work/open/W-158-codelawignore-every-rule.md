---
type: Handoff
name: W-158
description: "Arpit, 2026-10-30: one file for every rule's exceptions — .codelawignore, today limited to the two test-coverage rules, extended to any rule, each line with a reason and an expiry or forever."
item: W-158
filed: 2026-10-30
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-10-31
updated_by: "Claude Code (Opus 5.5)"
ball: arpit
---

# W-158 — `.codelawignore` holds the exceptions for every rule, not just test coverage

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Let each `.codelawignore` line name the rules it exempts | one file lists every exception in the repo | ✅ 2026-10-31 |
| 2 | Keep today's checks for every line: a reason, `expires=` or `forever`, `on_touch`, stale and incomplete lines flagged | no silent, endless exception | ✅ 2026-10-31 |
| 3 | The decider rules on reasons for `"off"` and scope exceptions in `codelaw.toml` | the config's exceptions held to the same bar | 🔴 decider decides |

**Arpit, 2026-10-30 (voice):** *"There could be one file where we can list all of these down."* After hearing the 10-26 design: *"Perfect, then we are good."*

**Model:** Sonnet: SR-BASELINE decision 12 already defines the line, its checks and its tests. This widens the scope from two rules to any rule.

## Goal

`.codelawignore` is the one file for file- and function-level exceptions to **any** rule. Every line carries the rules, a reason, and an expiry or `forever`. Every line is listed by `codelaw exceptions`.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rule ids on the line | a line is `<target> <rule-id>[,<rule-id>…] [keys]: <reason>`. The target is a glob or `<path>#<qualified name>`; a rule id may be a namespace (`js/*`). `#` opens a comment only at line start | ✅ 2026-10-31 |
| 2 · old lines | a line with no rule id keeps meaning `graph/untested-function` and `graph/function-coverage` for one minor release, with one warning naming the new spelling (as `[[overrides]]` → `[scope.<name>]`) | ✅ 2026-10-31 |
| 3 · same bar | `codelaw/incomplete-ignore` and `codelaw/stale-ignore` cover every line. An unknown rule id is incomplete and exempts nothing. `require_expiry`/`require_on_touch` move from `[coverage.ignore]` to a section that is no longer coverage's (name it in SR-CONFIG); the old keys stay aliases for one minor release | ✅ 2026-10-31 |
| 4 · `on_touch` for any rule | `on_touch=error|warn|keep` and `on_touch_scope=file|function` behave as for coverage. `function` needs a JS, TS or Python function, otherwise it's a load error naming the line | ✅ 2026-10-31 |
| 5 · listed and served | `codelaw exceptions` and MCP `list_exceptions` list each line once per rule, kind `codelawignore`; the LSP shows an exempted finding as the baseline's are shown (SR-EDITOR) | ✅ 2026-10-31 |
| 6 · tests | `ignore_file.rs` and a CLI test on real repos: a lint rule, a format-free rule, a namespace, a function target, stale and incomplete lines, an old-spelling line | ✅ 2026-10-31 |

## Built (2026-10-31)

- **Parser** (`crates/codelaw-baseline/src/ignore_file.rs`): rule ids and `ns/*` after the target; a line with none is the old spelling (`LEGACY_RULES`).
- **Engine** (`crates/codelaw-engine/src/codelawignore.rs`, new): load with id checks (unknown id, empty namespace, `codelaw/` id → incomplete; a function target outside JS/TS/Python → load error naming the line); one warning for old-spelling lines; the exemption stage in `check`, `fix`, watch and `check_snippet`, before the baseline; `fix` applies no fix to an exempted finding; the line findings (incomplete, unreadable, expiring/expired, stale — a glob is stale against every file under the root) now run whenever the file exists; exceptions rows per named rule. `new_code.rs` reads the same lines per test rule.
- **Config**: `[codelawignore]` (SR-BASELINE, SR-CONFIG §1, schema, root only); `[coverage.ignore]` read with a deprecation warning; both at once is exit 2.
- **Editor**: `data.ignored` from the LSP, a `.codelawignore` chip and meta line on the card; the VS Code extension draws `ignored` as it draws `baseline`.
- **Judged when building:** no `info` finding for a broad line (`** js/*`) — it is listed by `codelaw exceptions` with its reason and expiry like any other line (SR-BASELINE decision 12).

## Open question — the decider

**Reasons in `codelaw.toml`.** An inline `codelaw-ignore` and a `.codelawignore` line must give a reason. A rule set to `"off"` in `[rules]` or in a scope needs none. Recommended: a `reason` key on a rule entry that turns a rule off (`"py/no-print" = { severity = "off", reason = "CLI tool: print is the output" }`), with `[rules] require_reason` (default `false`, so no existing config breaks) to make it mandatory. Rule on it, or leave `codelaw.toml` as it is.

## Blockers

None for DoD 1–6. The open question holds only itself.

## Record affected

SR-BASELINE decision 12 (rewritten from "the test rules of new code" to every rule; it says "a general per-rule ignore file is a separate decision", and this is that decision); SR-CONFIG (where `require_*` lives); SR-LINTER decision 6 (the inline comment stays, and the file is its file-level twin).

## Hazards

- **The baseline is not this.** The baseline grandfathers counts with a deadline. `.codelawignore` exempts named targets. Keep both, and keep them distinct in `codelaw exceptions`.
- **A broad glob plus a broad rule** (`** js/*`) is a silent off-switch. Consider an `info` finding for a line that exempts more than N files (open, judge when building).

## Reference

- [SR-BASELINE](../../records/0104_baseline-and-deprecations.md) decision 12: the file today.
- `crates/codelaw-baseline/src/ignore_file.rs`: the parser and checks.
- `crates/codelaw-cli/tests/tests_for_new_code.rs`: today's tests.
