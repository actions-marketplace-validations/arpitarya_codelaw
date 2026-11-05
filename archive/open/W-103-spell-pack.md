---
type: Handoff
name: W-103
description: "Arpit, 2026-10-27 (Cowork, voice): build spelling into codelaw as its own rule pack — a known-typo check on by default, a dictionary check opt-in, codelaw's own word data from SCOWL, VarCon and the MIT software terms. Design of record: SR-SPELL."
item: W-103
filed: 2026-10-27
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: done
---

# W-103 — the spell pack: codelaw's own spell checker, better than CSpell

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Vendor SCOWL, VarCon and the software-terms list at a fixed release; record each pin in L11 §3 (D-42–D-44) and the notices in `packs/spell/NOTICE` | licence-clean, auditable sources | ✅ licences read from each source (SCOWL's `Copyright`, VarCon's README, the MIT `LICENSE`): all notice-only or MIT; SCOWL 2020.12.07 and software terms 5.4.8 pinned by SHA-256 in `spell-data.py` and L11 §3, notices in `NOTICE` and `licenses/`; VarCon 2020.12.07 pinned in `spell-data.py` and L11, its notice in `NOTICE` and `licenses/VarCon-Copyright.txt` (2026-10-28) |
| 2 | `scripts/spell-data.py` writes the dictionary, the locale table and the generated known-typo list into `packs/spell/data/spell/`; same input, same bytes | codelaw's own word data | ✅ `typos.json`, `words.json` and `locale.json`; `spell-data.py check` proves the same bytes (2026-10-28) |
| 3 | Measure: entries in the generated typo list, and the bytes the data adds to the npm tarball and the PyPI wheel; if JSON is too large, put a compact format to the decider (SR-RULE-PACKS decision 3) | the size question answered with numbers | 🟡 typo list: 188,151 entries, 3.53 MB JSON, pack 625 KB tgz / 636 KB wheel, +9 ms to load (SR-SPELL decision 9); one JSON string searched in place, so no new format; with the dictionary (868 KB) and locale table (489 KB) the pack is 1.03 MB tgz / 1.04 MB wheel and loads in +1 ms more, so no split and no new format (SR-SPELL decision 9, 2026-10-28) |
| 4 | The engine side: per-language selection queries, the word splitter, the token skips, the six `native:` rules and their fixes, the `[spell]` config section | the check itself | ✅ selection, splitter, skips, `[spell]` and six `native:` rules (2026-10-28); `spell/inconsistent-term` a `scope: project` rule on the graph stage, `graph: { terms: inconsistent }` (SR-PROJECT-GRAPH decision 10, 2026-10-28) |
| 5 | The pack: `packs/spell/` with `spell/recommended` and `spell/strict`, rule files with cases, README; L6, SR-RULE-PACKS and SR-PACKAGES amended to eighteen packs | installable as `@codelaw/rules-spell` / `codelaw-rules-spell` | ✅ the pack, both presets, seven rules with cases; L6, SR-RULE-PACKS, SR-PACKAGES, SR-RULE-FORMAT and every count at eighteen (2026-10-28) |
| 6 | Measure the speed: three CI `perf` runs on `main` with the pack on; put the budget to the decider by L8's method | the L8 budget | ✅ 2026-11-05 — set at **2,500 ms** by the decider (the method gave 2,152): `perf/budgets.toml`, L8, SR-SPELL decision 9 |

**Arpit, 2026-10-29 (voice docket; written on *"file it."*):** recommended measuring the pack as its own `spell` operation so the ruled budgets do not move: *"We can, but add an item in the backlog so that we can have a shared benchmark."* — its own operation now; B-062 holds the shared benchmark.

**Arpit, 2026-10-27 (voice):** *"There is a tool called C-Spell. Research it and tell me if that can be built into CodeLaw. A similar feature, a better feature, would be even better."* Then, ruling the forks: *"Let's go with B."* · *"Approve both."* · *"Approve it."* · *"want it as a separate package."* · *"Go with the recommendation."* · *"go ahead"*. The forks: [the compare doc](../../work/compare/spell-checking.compare.md).

**Model:** Opus: it amends a law (L6's pack count) and the generated typo list's quality is judged by people, not caught by a test; close to Sonnet, because SR-SPELL fixes the rules and their fix classes.

**Ruled 2026-11-05 — the `spell` budget is 2,500 ms. Arpit (voice):** *"Let's set it."* → *"Let's set it as 2500 milliseconds."*; then (text) *"file it."* Above L8's method (2,152 ms), and recorded as his number, not the formula's. A `spell` p95 over 2,500 ms now fails CI, and W-126's `perf spell` job is gated.

**Arpit, 2026-11-05 (voice docket):** *"Go with the recommendation."* — (1) the next agent session reads the `spell` p95s from the three `perf` runs (their job summaries need a GitHub sign-in; the desktop app's browser pane is signed in) and puts the budget to him by L8's method; (2) **`typos` deferred:** its L11 ratification (item 8) waits — not needed to finish the pack.

## The `spell` budget, proposed by L8's method (2026-11-05, Claude Code)

Read from each run's `perf-bench` artifact (`bench.json`, `gh run download <id> -n perf-bench`), `ubuntu-24.04`, 4 threads, `codelaw bench` 10 runs with the first discarded, the binary each `perf` job built with `cargo build --release`:

| run | commit | codelaw | files · findings | median | p95 |
|---|---|---|---|---|---|
| A = `ci` 41005346138 | `7604e4a6` | 0.4.0 | 1,004 · 12 | 1,320.8 ms | **1,383.3 ms** |
| B = `ci` 41006814384 | `7fafb2f4` | 0.4.0 | 1,004 · 12 | 1,353.2 ms | **1,434.2 ms** |
| C = `ci` 41007205048 | `c4d519c7` | 0.4.1 | 1,004 · 12 | 1,370.3 ms | **1,429.5 ms** |

- **The method** ([L8](../../records/0010_LAW-8-performance-budgets.md) decision 4, ruled 2026-10-25): min(design target, 1.5 × the worst p95), rounded up to the next whole ms. Worst p95 1,434.2 ms × 1.5 = 2,151.3 → **2,152 ms**.
- **No design target** names `spell` (SR-ARCHITECTURE decision 29, SR-SPELL decision 9), so the guard that held back `fix_one_file` and `hook_pre` does not apply: the method gives a number.
- **Proposed: `budget = 2152`**, `measured_p95 = 1434.2`, `set_on` the day of the ruling. Not set: only a ruling sets it (decision 4). Once set, a `spell` p95 over 2,152 ms fails CI (decision 7), and W-126's `perf spell` job becomes gated with no change to `ci.yml`.
- **For the ruling:** the runner measures 2.4 × the loaded Mac's 584 ms (four threads against ten); the same 12 findings on all three runs, so the work did not move. From W-126 (2026-11-05) `perf` runs on `build linux-x64-gnu`'s binary (the same release profile, linked by maturin with zig), not a `cargo build --release` of its own; W-126's measurement runs give `spell` on that binary as a cross-check (W-126 §Measured).

## Goal

Spelling in comments, docs, prose strings, Markdown, keys and declared names, in all nine L3 languages — with unattended fixes for known typos and an opt-in dictionary check — built only from codelaw's own code and the three ratified sources ([SR-SPELL](../../records/0116_spell.md)).

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · data | `packs/spell/data/spell/` is written by `scripts/spell-data.py` (stdlib only) from D-42–D-44 at the pins in L11 §3; a second run changes no byte; `NOTICE` names each source and version | ✅ all three tables; `spell-data.py check` passes; `NOTICE` names SCOWL, VarCon and the software terms (2026-10-28) |
| 2 · rules | the six rules of SR-SPELL decision 5, each with its fix class, cases and a `codelaw test` pass; every `manual` fix carried by a skill (L4) | ✅ `spell/known-typo` (107 `codelaw test` checks), `spell/known-typo-identifier` (48) and `spell/known-typo-key` (28), `spell/unknown-word` (90), `spell/locale-variant` (94), `spell/suspicious-word` (6), `spell/inconsistent-term` (`suggest`, 94): 467 checks (2026-10-28) |
| 3 · scope | a fixture repo per L3 language: a typo in a comment, a string, a declared name and a key is reported once; the same word in a URL, a hash, a JSON value that is an id, or at a name's uses is not | ✅ 2026-10-28: `fixtures/spell/repo` (twelve files, nine languages); `crates/codelaw-cli/tests/spell.rs` asserts the 38 findings, each once, and none on a URL, a hash, an id value, a code span, a link target, an HTML class or a name's uses |
| 4 · fixes | `codelaw fix` on the fixtures fixes the one-correction known typos in prose and file-local names, and changes nothing exported | ✅ 2026-10-28: the same test runs `codelaw fix`: prose and file-local names fixed (29 findings), the 9 left are exported or module-level names, a member, keys, a class and an id, all untouched; a second `fix` changes nothing |
| 5 · packaging | `@codelaw/rules-spell` and `codelaw-rules-spell` build in CI byte-identical; `init` does not add the pack; a missing pack names both install commands | ✅ 2026-11-05: `build-packs.py --verify target/release/codelaw` (0.4.3) builds `codelaw-rules-spell-0.4.3.tgz` (1,033,879 B) and `codelaw_rules_spell-0.4.3-py3-none-any.whl` (1,052,102 B); the same 27 files in both; SHA-256 of `typos.json` `a01c9084…`, `words.json` `041b89ee…`, `locale.json` `148305ec…` equal in the tarball, the wheel and the tree; `check` output on `fixtures/l3` byte-identical from either install. The check that keeps it so already exists: `build-packs.py`'s `check_parity` and `package.py check` (pack parity) in CI's `package` job, green on `main` in `ci` 41945322410 (`9ae07264`). `init` *adds* the pack since W-112 (SR-SPELL decision 1, reversed by the decider 2026-10-28), so that clause is superseded; the missing-pack message names both installs (`pack.rs` test) |
| 6 · measured | the typo list's entry count, the data's bytes per package, and the spell pass's time on the pinned corpus are written into SR-SPELL decision 9; the budget is the decider's ruling | ✅ the budget 2,500 ms (decider, 2026-11-05); before it: entry count, bytes per package and the corpus timing (2026-10-28, a loaded machine: `spell/recommended` 6–10 s of rule time summed, 3.3–6.0 s wall on 2,500 files) in SR-SPELL decision 9; the budget left (step 7) |
| 7 · records | SR-SPELL `built: yes`; L6, SR-RULE-PACKS §1 and decision 4, SR-PACKAGES at eighteen packs, sixteen on npm and eleven on PyPI; README, CHANGELOG, GLOSSARY, `docs/rules/` regenerated | ✅ all at eighteen (now counted by later items); `docs/rules/` regenerated (2026-10-28); SR-SPELL `built: yes` in the register 2026-11-05, decision 1 gains *Packaging, as built* |

## Left (exact, after the sixth increment of 2026-10-28)

1. ✅ 2026-10-28: `spell/known-typo-identifier` (`auto`, JS/JSX/TS/TSX/Python, 48 `codelaw test` checks) and `spell/known-typo-key` (`manual`, JSON/YAML/TOML/CSS/SCSS/HTML, 28 checks), both in `spell/recommended`; one rule per fix class, as SR-SPELL decision 5 *as built* says. Not built: the `suggest` tier naming other files (needs the project graph).
2. ✅ 2026-10-28: the dictionary — SCOWL to size 60 plus the software terms, 92,758 words, 868 KB, +1 ms to load; no split or new format needed (SR-SPELL decisions 6 and 9 *as built*).
3. ✅ 2026-10-28: `spell/unknown-word` (`suggest`, in `spell/strict`) and `[spell]` (`locale`, `words`; SR-CONFIG §1 and decision 2, the schema, `config` trace); `[spell] words` excuses every rule but `suspicious-word`.
4. ✅ 2026-10-28: `spell/suspicious-word` (`manual`, `spell/strict`) and `spell/locale-variant` (`auto`, VarCon's pairs in `locale.json`, NOTICE and `licenses/` gain VarCon) built. ✅ 2026-10-28 (sixth increment): `spell/inconsistent-term` (`suggest`, `spell/strict`) — a `scope: project` rule, `graph: { terms: inconsistent }` with `fix: { do: [respell] }` (SR-PROJECT-GRAPH decision 10), run by the engine on the graph stage (`project.rs`): while it is on, the graph's sources carry every file's text; each file's prose words of VarCon's pairs (`codelaw_native::spell::term_uses`) are cached by content and the rule set, the watch path re-reads a saved file's; each use of the spelling the project writes less is reported, the other as the edit (American on a tie); `[spell] words` not counted, nothing under `[spell] locale`. 94 `codelaw test` checks; `fixtures/spell/repo` gains `docs/orders.md`, `src/orders.js`, `config/orders.yaml`, and `tests/spell.rs` two tests (check cold and warm, an edit moving the verdict, `fix` vs `fix --unsafe`; `check --watch` following a save). Corpus: **107** (`packs/spell/corpus.toml`), the eleven repositories as one project; 61 per repository, reviewed: real mixes, with 6 in Django's generated migrations (`Rouble`, a currency's name) and 2 in a JSON data file (SR-SPELL decision 9).
5. ✅ 2026-10-28: DoD 3 and 4 — `fixtures/spell/repo` and `crates/codelaw-cli/tests/spell.rs`.
6. ✅ 2026-10-28: the pinned corpus — `spell` on eleven repositories (2,500 files); the first run's 1,010 `spell/recommended` findings reviewed (almost all lorem ipsum, translated READMEs, contraction stems, lockfile package names, jargon), the English test, template masking, lockfile and dependency-key skips and eight `!word` exclusions added; the snapshot is 125 / 3 / 1 (`known-typo` / `-identifier` / `-key`) and 8,948 `unknown-word` (SR-SPELL decision 9 *the pinned corpus*). Every rule stays `preview`. **Not done:** the comparison with `typos`' count — `typos` is not installed and is not added (L11); promoting a rule out of `preview` is `corpus.py promote`'s, under SR-RULE-AUTHORING's criterion.
7. 🟡 2026-10-30: the `spell` operation is built as ruled (L8 decision 1's table, SR-CLI-MCP decision 10, SR-SPELL decision 9 *the operation, as built*, `fixtures/perf/README.md`); CI's `perf` job runs it with no `ci.yml` change. **Next:** three `perf` runs on `main` (the landing push, then the nightly or a dispatch), decision 4's method applied, the number to the decider. Before: step 6: three CI `perf` runs on `main` with the pack on, then the budget to the decider (L8). **Found 2026-10-28:** the `perf` workload (`crates/codelaw-cli/src/bench/workload.rs`, `CONFIG`) does not load the spell pack, and adding it there would move every set operation's p95 against budgets already ruled — so the decider first rules *how* the pack is measured (inside the shared workload, or as its own operation; the inbox row in OPEN-WORK), then its budget from three runs.
8. 🔴 the decider. Not in this item's DoD, named so nothing is lost: the `typos` count comparison (SR-SPELL's reopen-trigger) needs `typos` ratified in L11 (a CI-only tool), his call. `known-typo-identifier`'s `suggest` tier naming other files needs the project graph's names, which the graph reads for JS, TS and Python only: not built, not in the DoD.

## Blockers

None. **Closed 2026-11-05 (Claude Code):** every DoD row met; archived. Outside the DoD and not lost: the `typos` count comparison (item 8, needs `typos` ratified in L11 — the decider's) and `known-typo-identifier`'s `suggest` tier naming other files (needs the project graph's names), both in SR-SPELL's *as built* text.

## Record affected

[SR-SPELL](../../records/0116_spell.md) (all) · [L6](../../records/0008_LAW-6-python-and-node-packages.md) (the pack count, on the ruling above) · [SR-RULE-PACKS](../../records/0101_rule-packs.md) §1, decisions 4 and 6 · [SR-PACKAGES](../../records/0107_packages.md) · [SR-CONFIG](../../records/0110_config.md) decision 2 (`[spell]`) · [L11 §3](../../records/0013_LAW-11-dependencies.md#3--the-list) (the pins) · [SR-PROJECT-GRAPH](../../records/0103_project-graph.md) decision 10 and [SR-ARCHITECTURE](../../records/0108_architecture.md) decision 4 (`spell/inconsistent-term` on the graph stage).

## Hazards

- **Never copy `typos`', codespell's or Wikipedia's lists**, not even to compare in a test fixture — SR-SPELL's veto condition greps for them.
- A generated typo that is a real word in some domain (a surname, a chemical) makes an `auto` fix wrong: keep only variants with one source word that SCOWL's largest list does not hold, and let the corpus run find the rest before any rule is `stable`.
- Renaming an identifier across files is a refactor: `auto` only for a file-local, unexported name (SR-SPELL decision 5).
- Data size: SCOWL with every form written out may be several MB as JSON — measure (step 3) before choosing a format.
- `HashMap` order, Unicode case folding and locale sorting can make output differ by platform (L6: identical output) — sort everything, NFC + simple fold.

## Reference

- [SR-SPELL](../../records/0116_spell.md) · [the compare doc](../../work/compare/spell-checking.compare.md) · [L11 §3](../../records/0013_LAW-11-dependencies.md#3--the-list)
- SCOWL and VarCon: <http://wordlist.aspell.net/> · software terms: <https://www.npmjs.com/package/@cspell/dict-software-terms>
