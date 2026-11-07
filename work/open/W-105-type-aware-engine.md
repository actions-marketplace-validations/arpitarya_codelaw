---
type: Handoff
name: W-105
description: "Type-aware TypeScript (W-101): the codelaw-types helper, the type: clause, ts/type-error and the ts rules — preview on the 7.1 RC (2026-11-10), stable after GA (2026-11-24) — and the TypeScript version watch (was W-106). Merged 2026-10-30."
item: W-105
filed: 2026-10-27
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-105 — Type-aware TypeScript, and its version watch

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | `codelaw-types` helper (codelaw's own JS) on `typescript/sync`: loads the `tsconfig` program once; codelaw's versioned stdin/stdout protocol; disposes snapshots | type answers | ✅ 2026-10-28 — `crates/codelaw-types` (`helper.mjs` run as `node --input-type=module --eval`, protocol 1: `hello`/`check`/`changed`/`refresh`/`close`, UTF-8↔UTF-16 offsets, an overlay snapshot for unsaved text, every snapshot disposed) |
| 2 | Engine: find Node and the project's `typescript`; < 7.1 or absent → type rules skipped with a note; the `type:` clause; cache key gains TS and protocol versions; report names both | rules can ask | ✅ 2026-10-28 — discovery from the file's folder up (`CODELAW_TYPESCRIPT` as the tester's fallback), the ≥ 7.1 gate and the tested range `>=7.1.0-beta, <7.2.0`, the `type:` clause (SR-RULE-FORMAT 3f), `--format json` `types {protocol, typescript[], notes[]}` and the summary line; a file a type rule checks is **not cached** while a program answers (its imports are outside any per-file key — SR-TYPES decision 12 amended) · ✅ 2026-10-28 `codelaw --version --json`: `codelaw`, `types {protocol, tested, typescript {version, path, api, tested} \| null, node}` from the current folder, nothing started (`codelaw_engine::types_version`; `tests/types.rs` `version_json_names_the_protocol_and_the_project_typescript`; SR-CLI-MCP decision 1) |
| 3 | Warm: daemon, LSP and `--watch` keep the program (`snapshot.update`); `hook` / `hook --pre` use it warm or skip with a note | L8 holds | ✅ 2026-10-28 — helpers kept per process (≤ 4), `refresh` re-reads files whose mtime moved, `--watch` sends `changed` and re-checks every typed file; `hook`/`hook --pre` run with `no_cold_types` (`crates/codelaw-cli/tests/types.rs` `a_hook_uses_a_warm_program_and_never_a_cold_one`) · ✅ 2026-10-28 the long `--watch` memory test: `a_long_watch_does_not_grow_with_its_rechecks`, 100 re-checks, the watch and its helper within 15 % of their size after 20 (139 → 146 MiB on macOS; with `replaceBase`'s `dispose` removed, 144 → 191 MiB and the test fails) |
| 4 | `ts/type-error` (TS diagnostics; TS code fixes as `suggest`) | TypeScript's own errors, once | ✅ 2026-10-28 — `native: ts-type-error`, syntactic + semantic errors and warnings as `TS<code>: …`; `manual`, since 7.1's API has no code fixes |
| 5 | First rules in `packs/ts`, `preview`, `ported_from` set: `no-floating-promises`, `no-misused-promises`, `await-thenable`, the `no-unsafe-*` family, `switch-exhaustiveness-check`; fixtures | the important ones | 🟡 2026-10-28 — `ts/type-checked`: `no-floating-promises`, `await-thenable`, `no-unsafe-call`, `no-unsafe-member-access`, `no-unsafe-return`, `type-error`, each passing `codelaw test` under 7.1.0-beta; `fixtures/types` fires each once and an oxlint port is shown once. `no-misused-promises`' condition check is TS2801 (`ts/type-error`), so not built (SR-TYPES decision 10) · ✅ 2026-10-30 the four questions SR-TYPES decision 4 was amended for (ruled 10-29, *"Add it."*): `unknown`, `typed-target`, `promise-to-void`, `non-exhaustive`, and their rules, `preview`, each firing once in `fixtures/types`: `switch-exhaustiveness-check`, `no-unsafe-argument`, `no-unsafe-assignment`, `no-misused-promises` (`checksVoidReturn` only); `await-thenable` no longer reports `await` on `unknown`. `codelaw test packs/ts` 31 rules · 333 checks green under 7.1.0-beta; `tests/types.rs` 6/6 |
| 6 | **Release as preview on the 7.1 RC (2026-11-10)**; measure `check_types_*` on CI | early release | 🟡 2026-10-28 — `codelaw bench --typescript <dir>` times `check_types_cold` / `check_types_warm` on the type workload (the reference workload's 2,200 `.ts` files, one `tsconfig.json`, `ts/type-checked` only; digest `5341cd61a21e66c7`), the `perf` job installs `fixtures/types`' TypeScript and passes it, and both are `unset` in `perf/budgets.toml` with a `proposed` from one loaded-Mac run (cold p95 3955 ms, warm 3170 ms) · ⏳ the RC is not out (today 2026-10-28) · ✅ 2026-11-05 CI timed them three times on `main` (`perf`, runs 41005346138, 41006814384, 41007205048, all 2026-10-30, the old helper): cold p95 11 383 / 12 219 / 13 102 ms, warm 8 052 / 8 681 / 9 164 ms · ✅ 2026-11-05 profiled and sped up — see *Profiling* below |
| 7 | After 7.1 GA (2026-11-24): re-run on GA, then the rules go to promotion (W-100's criterion); port the rest of the list | stable | ⏳ |
| W-106 | TypeScript version watch — merged in; its steps under [§ W-106](#merged-in-w-106) | one item, not two | ⏳ — see § |

**Merged 2026-10-30 (Arpit, text):** *"There are too many work items. See if we can review all the work items and merge them into few of them."* → *"go ahead, merge them."* Merged in: W-106.

**Arpit, 2026-10-29 (voice docket; written on *"file it."*):** asked whether agents may extend SR-TYPES decision 4's vocabulary for the four rules (exhaustive `switch`, `any` into a typed target, a promise where a `void` callback is expected, `await` on `unknown`): *"Add it."* Then, recommended to build the four as previews and to profile the slowest type rule before any budget is put to him (he said on W-104 *"make sure it is as fast as possible"*): *"Let's go ahead and build it out."*

### Left (exact)

1. ~~The four type questions~~ — built 2026-10-30 (step 5 above).
2. ~~Profile the slowest type rule~~ — done 2026-11-05 (*Profiling* below): no rule is slow; the helper's one call per question was. Batched; findings identical.
3. L8: the three `perf` runs on `main` exist (2026-10-30, worst p95 cold 13 102 ms, warm 9 164 ms) but timed the **old** helper. Decision 4's method on them would give cold 19 653, warm 13 746 — budgets for code that is gone. **2026-11-05, after the push:** the three `ci.yml` runs on `d5006de0`/`3b53d1a8` (42053047689, 42053750244, 42053908300) skip `perf` — `linux-arm64-musl`'s build is red and `perf-op` needed every build leg. W-126 fixed that on `main` (`9b8ca7b7`, `perf-op` on `!cancelled()`), not yet on GitHub. Next: that commit pushed (the decider's), three `perf` runs on the batched helper (W-126's runs serve both), then the inbox row with decision 4's method applied (no design target exists for either).
4. The type rules' corpus counts stay 0, and the corpus job cannot make them real alone. Level 4 checks each corpus file alone (the rule tester's root is not the repo), and the corpus repos have no `node_modules`, so every import reads as `any`. Tried 2026-10-28 file by file; **re-run 2026-11-05 with each repo as its own program** (its `tsconfig.json`s, 7.1.0-beta, the batched helper): zustand · jotai · full-stack-fastapi-template · ionic-framework give `no-unsafe-call` 2 304 · 5 934 · 1 185 · 28 557, `no-unsafe-member-access` 1 209 · 3 310 · 1 173 · 19 612, `type-error` 187 · 1 510 · 847 · 9 989 (TS2307 *cannot find module* 41 · 180 · 194 · 1 660; TS7026 JSX without React's types 114 · 1 093 · 390 · 927), `no-unsafe-return` 84 · 374 · 188 · 425, `no-unsafe-argument` 47 · 80 · 6 · 913, `no-unsafe-assignment` 41 · 84 · 50 · 242, `no-floating-promises` 4 · 31 · 1 · 150, `no-misused-promises` 0 · 0 · 0 · 20, `switch-exhaustiveness-check` 0 · 0 · 0 · 3, `await-thenable` 0 · 0 · 0 · 2. The repo's program is not enough: the unresolved dependencies still make the counts. Real counts need level 4 to check a typed rule's files in their repo's program (codelaw-test, SR-RULE-AUTHORING) **and** each corpus repo's dependencies installed in CI — an L11 question for the decider, not asked yet; it is worth asking only before promotion (step 7).
5. Step 6: re-pin `fixtures/types` to `7.1.0-rc` on 2026-11-10, rerun, release as preview; step 7 after GA (2026-11-24).
6. ~~The version watch's dispatch proof~~ — done 2026-11-05 after `main` was pushed (`3b53d1a8`): a plain dispatch changed nothing (run 42054038797: 7.1.0-beta is pinned and npm's newest with the API); `version=7.1.0-dev.20261105.1` opened one re-pin PR, #7, and dispatched `ci.yml` on its branch (run 42054853732); `version=7.1.0-dev.20260708.3` — a nightly before the API the helper uses — failed `tests/types.rs` and opened one issue, #6 (run 42054074700). #7 closed with its branch and #6 closed: a nightly is no pin. The RC re-pin (11-10) is the next real one.

### Profiling (2026-11-05)

The helper timed phase by phase on the type workload (2,200 files, `ts/type-checked`), through a scratch `node` on `PATH` that ran an instrumented copy of `helper.mjs`; the machine was loaded (load average 30–90 from parallel sessions), so the shares are the finding, not the milliseconds. One cold run, the old helper: 2,376 `check` requests, 142,358 questions (100,041 `any`, 31,621 `typed-target`, 10,696 `promise-to-void`) — `any` 40.2 s, `promise-to-void` 13.2 s, `typed-target` 10.0 s, `getDefaultProjectForFile` 14.7 s, `getSourceFile` 3.0 s, `nodeAt` 1.8 s, semantic diagnostics 4.0 s, syntactic 0.9 s, of 97.6 s. Every question was its own synchronous round trip to TypeScript's Go process: type checking itself (the diagnostics) was under a tenth.

Changed in `crates/codelaw-types/src/helper.mjs` (protocol unchanged, SR-TYPES decision 2 amended): each file's questions and its diagnostics run as one `api.batch` of the API's `.gen` calls (one message per round of calls, not per call); a file's project comes from the snapshot's own file lists when one loaded project under its `tsconfig.json`'s folder holds it. After: `getDefaultProjectForFile` once per run (was once per file); the batches 25.7 s of a 50.4 s run on the same loaded machine, most of it TypeScript's own checking, which a batch's first type call triggers. Same-machine `check` cold, back to back: old 104.9 / 65.0 / 85.4 s, new 32.1 / 27.1 / 48.8 / 31.4 s; warm (the daemon's second run) 8.8 s. The 576 findings on the workload identical, byte for byte; `codelaw test packs/ts` 64 rules · 737 checks and `cargo test --test types` 6/6 green. The next lever, if CI asks for one: several files per request (a protocol change), since a warm run is now mostly one round trip per file.


**Arpit, 2026-10-27 (voice), W-101:** rulings R1–R8 in [the proposal](../proposals/type-aware-rules.md) §3 — TypeScript 7.1's own API through the project's installed copy, Node at runtime, rules in the `ts` pack, preview on the 7.1 RC, *"Go with the recommendation"* on version checks.

**Model:** Opus — a new process boundary and a protocol.

Built against 7.1 Beta (2026-10-20) until the RC lands. First landing 2026-10-28 (Claude Code): steps 1–4 and five first rules. Second landing 2026-10-28 (Claude Code): `--version --json`, the `check_types_*` bench and its CI wiring, the long-`--watch` memory test; the corpus counts tried and found not real. The item stays open on *Left* above, its next step the decider's.

## Definition of done

- Steps 1–6 done, with a fixture repo checked under 7.1 with each first rule firing and deduping against `adapter/oxlint/typescript/*`; step 7 may close as its own item.

## Record affected

SR-TYPES (W-104; amended with the as-built decisions 2–5, 7–10 and 12, and its Ownership rows, 2026-10-28), SR-RULE-FORMAT 3f, SR-RULE-PACKS (the `ts` row names `ts/type-checked`; the pack's README drops "out of scope" for `*-type-checked`), SR-PACKAGES decision 1 (`codelaw-types` in the engines layer), L11 §3 D-45 (the pinned tree). Second landing: SR-CLI-MCP decisions 1 and 10 (`--version --json`, the type workload), SR-TYPES decisions 2 and 12 and §Consequences, L8 §How it is enforced and §Consequences (status lines only), SR-PACKAGES (the `perf` job's TypeScript), SR-ARCHITECTURE (`types_version`). Third landing (2026-11-05): SR-TYPES decision 2 (how the helper asks: one batch per file, the project from the snapshot's file lists) and §Consequences (the corpus counts re-run as each repo's program).

## Hazards

- The 7.1 API is a preview: keep every call behind the helper so a break is one file.
- Forgetting `dispose` leaks on the Go side — test a long `--watch` run for memory (done: `a_long_watch_does_not_grow_with_its_rechecks`).

## Merged in: W-106

<a id="merged-in-w-106"></a>

**W-106 — TypeScript version watch**, merged into W-105 on 2026-10-30; its file is archived at [`archive/open/W-106-typescript-version-watch.md`](../../archive/open/W-106-typescript-version-watch.md). As it stood:

### At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | A tested TS range in SR-TYPES and the helper; outside it, type rules run with a warning (the adapter pattern) | users on newer TS keep working | ✅ 2026-10-28 — `codelaw_types::TESTED` `>=7.1.0-beta, <7.2.0`, SR-TYPES decision 7 (step 2 of W-105's table) |
| 2 | `ci.yml`: the type-rule fixtures against the latest 7.1 on every change (blocking) | no regressions | ✅ 2026-10-30 — the `test` job reruns `tests/types.rs` and `codelaw test packs/ts` on npm's newest 7.1 release (`scripts/typescript-watch.py latest --below 7.2`; a range would pick a `7.1.0-dev` nightly) when it is not the pin; skipped with a reused verdict, so a push between nightlies may miss a new release |
| 3 | Nightly: the same against `typescript@next`, report only | early warning | ✅ 2026-10-30 — `ci.yml` `types-next` (schedule and dispatch, `continue-on-error`, the summary says pass or fail) |
| 4 | `typescript-bump.yml`, modelled on `fux-bump.yml`: a new TS release → run the fixtures → PR widening the range, or an issue naming what broke | nobody has to watch | 🟡 2026-10-30 — written: `typescript-watch.py latest`/`bump` (tried on a scratch copy: beta→rc re-pins, rc→7.2.0 widens to `<7.3.0`, 7.0.0 refused) and the workflow (one PR or one issue). ⏳ the definition of done — a dispatch with an older and a newer version — needs it on GitHub and the *Allow GitHub Actions to create … pull requests* setting (the decider's, as for `fux-bump`) · ✅ 2026-11-05 the definition of done: a plain, a newer and a breaking dispatch — nothing, PR #7, issue #6 (*Left* 6) |

**Arpit, 2026-10-27 (voice), W-101:** rulings R1–R8 in [the proposal](../proposals/type-aware-rules.md) §3 — TypeScript 7.1's own API through the project's installed copy, Node at runtime, rules in the `ts` pack, preview on the 7.1 RC, *"Go with the recommendation"* on version checks.

**Model:** Sonnet — CI wiring on a known pattern.

### Definition of done

- A dispatch with an older and a newer TS version each produces the right PR or issue (as W-91 proved `fux-bump.yml`).

### Record affected

SR-TYPES (W-104) for the range; SR-PACKAGES if `ci.yml` jobs are named there.

### Hazards

- The nightly must never block `main`.
