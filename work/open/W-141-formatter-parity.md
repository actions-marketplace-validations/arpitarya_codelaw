---
type: Handoff
name: W-141
description: "The formatter, closer to Prettier where it measurably differs (W-138's corpus: CSS 42%, HTML 17%) — including Oxc's native formatters, measured and put under L11 (was W-148). Merged 2026-10-30."
item: W-141
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-141 — The formatter, closer to Prettier (and Oxc's formatters)

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | CSS: a comment's `*/` and the next selector stay on separate lines | a defect gone | ✅ 10-30 |
| 2 | HTML: void elements written as Prettier writes them (`<meta charset="utf-8" />`) | most of the HTML gap | ✅ 10-30 (HTML 18.4% → 68.7%) |
| 3 | Rerun the measurement; then Markdown code blocks, CSS `calc()` breaks, JS/TS comment placement | each language within 2% of Prettier | 🟡 rerun done; what is left is inside Biome and markup_fmt — see § Built |
| 4 | The corpus and the comparison kept as a repeatable check | the number cannot slip back unseen | ✅ 10-30 |
| W-148 | From Oxlint: Oxc's formatters, 18 rules, 38 bug rules — merged in; its steps under [§ W-148](#merged-in-w-148) | one item, not two | ✅ oxfmt measured; Oxc's crates ratified 2026-11-05 (L11 D-52); ✅ built behind the formatter 2026-11-05 (§ Built 2026-11-05) · 🟡 step 4 waits on W-139 |

**Ruled 2026-11-05 — Oxc's formatter crates ratified under L11. Arpit (voice):** *"Approve it."*; then (text) *"file it."* `oxc_formatter_css`, `_markdown`, `_yaml`, their parsers, `oxc_formatter_core`, `oxc_allocator` (MIT, a pinned git revision) are [SR-LAW-11](../../records/0013_LAW-11-dependencies.md) D-52. **Next (agent):** § W-148 step 1 — pin a revision (write it, the transitive count and its `members`/`tree` ledger lines into D-52, which `tests/dependencies.rs` reads), build them behind `codelaw format` for CSS, SCSS, YAML and Markdown, rerun `format-corpus.py`, and measure L8, the layout check and the binary size; then [SR-FORMATTER](../../records/0105_formatter.md) records what was built.

**Merged 2026-10-30 (Arpit, text):** *"There are too many work items. See if we can review all the work items and merge them into few of them."* → *"go ahead, merge them."* Merged in: W-148.

**Source:** [W-138's proposal](../proposals/rule-parity.md) §7 — `codelaw format` 0.4.0 against Prettier 3.9.10 (Ruff 0.16.9 for Python), default settings, byte-identical outputs: JSON 100%, Python 100%, JS 95%, TS 94%, YAML 89%, SCSS 83%, Markdown 73%, **CSS 42%, HTML 17%**.

**Model:** Opus (Claude Code): formatter changes in two printers, each checked against the corpus.

## Goal

SR-FORMATTER decision 5 says the defaults are Prettier's (Ruff's for Python). Where codelaw's output differs from Prettier's on real files, codelaw changes — except where a record says otherwise, and then the record says why.

Seen in the samples:

- **CSS** — `/* … */` followed by a selector prints as `*/ :root,` on one line (Primer CSS, `006.css` in the corpus); `calc(a + b)` sums break at different operators.
- **HTML** — `<meta …>`, `<input …>` and every void element: codelaw omits ` /`, Prettier writes it.
- **Markdown** — a fenced JS block's object is wrapped (`new Cacheable({` / `/* options */` / `})`) where Prettier keeps one line.
- **JS / TS** — comments inside calls and object literals placed differently.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · CSS defect | the comment/selector join fixed, with a case | ✅ |
| 2 · HTML void | Prettier's ` />`, or a record ruling for codelaw's form | ✅ |
| 3 · the rest | Markdown, CSS, SCSS, YAML, JS, TS each ≥ 98% identical on the corpus, or the difference named in SR-FORMATTER | ⏳ |
| 3b · Prettier keys | `[format]` keys for `arrowParens`, `bracketSpacing`, `jsxSingleQuote`, `quoteProps` (W-138 R3, ruled 2026-10-30); the other seven Prettier options stay at Prettier's default | ✅ `arrow_parens`, `bracket_spacing`, `jsx_quote_style`, `quote_props` (no `consistent`: Biome has none) |
| 4 · repeatable | the corpus (sources and licences listed, not vendored where large) and a script that prints the table — in `perf/` or `fixtures/`; not in CI unless cheap | ✅ `scripts/format-corpus.py` (pinned sources, fetched outside the repo) + `scripts/format-parity.py --corpus`; not in CI |

## Built 2026-10-30 (Claude Code, Opus 5.5)

**Corpus.** `python3 scripts/format-corpus.py` fetches pinned sources (Bootstrap 5.3.8, Bulma 1.0.4, Primer CSS 22.3.3 — npm; MDN learning-area, actions/starter-workflows, ESLint, typescript-eslint — git commits; MIT or CC0) into `~/.cache/codelaw-format-corpus`, never into the repo, and takes up to 150 files a language, evenly spaced; `python3 scripts/format-parity.py --binary <codelaw> --corpus <dir>/files --prettier 3.9.10 [--diffs <dir>]` prints the table. Its CSS is the 79 files W-138 measured (oxfmt's 88.6% reproduces exactly); its HTML, JS, TS and Markdown files differ from W-138's, so its numbers are not W-138's.

| language | files | 0.4.3 | after this change | oxfmt 0.75.0 |
|---|--:|--:|--:|--:|
| CSS | 78 | 73.1% | 74.4% | 88.6% |
| HTML | 147 | 18.4% | **68.7%** | 100% (Prettier itself) |
| SCSS | 150 | 80.7% | 80.7% | 86.0% |
| YAML | 149 | 89.9% | 89.9% | 97.3% |
| Markdown | 150 | 93.3% | 93.3% | 97.3% |
| JS · JSX | 139 | 97.1% | 97.1% | 97.8% |
| TS · TSX | 129 | 82.2% | 82.2% | 82.9% |

**Changed** (`crates/codelaw-format`): own-line CSS/SCSS block comments at statement level keep their line (whitespace only, so no token changes; a comment followed by another comment is left); HTML void elements as ` />`; `style=""` as `a: b; c: d`; a `<script>` body gets the width left after its indentation; the four Prettier keys (DoD 3b) in `[format]`, `--import-config` and `migrate biome`. `codelaw test` on `html`, `css`, `scss`, `security-html`: 0 failed, the one CSS advisory warning 0.4.3 already gives.

**What is left is inside the engines**, not codelaw's code: CSS/SCSS — Biome fills custom-property comma lists on one line, breaks `calc()` at every operator, spaces `a/b`; HTML — markup_fmt breaks inside an inline element (`<a\n  href>`) where Prettier moves the element to the next line, prints `<caption>` on one line and an empty `<style>` on two; YAML, Markdown, TS — not examined this round. Reaching 98% means changing engines, which is the W-148 question.

**W-148 step 1, as far as an agent may take it.** oxfmt 0.75.0 (npm, installed in a scratch folder outside the repo, deleted after) measured on the same corpus — the table's last column. On the CSS, SCSS, Markdown and YAML files (529, 3.5 MB) oxfmt took 0.23 s single-threaded against `codelaw format`'s 2.3 s (codelaw formats each file twice to verify; not like for like). Its native module is 7.0 MB against codelaw's 30.8 MB binary. **Not done:** `oxc_formatter_css`/`_markdown`/`_yaml` built behind codelaw's formatter — adding a git dependency, even on a branch, is the L11 question, and the machine was past its disk ceiling (13 GB of 14 approved) while seven agents built.

**For Arpit (L11):** ratify Oxc's native formatter crates (`oxc_formatter_css`, `oxc_formatter_markdown`, `oxc_formatter_yaml`, their parsers and `oxc_formatter_core`, `oxc_allocator`; MIT; git rev, `publish = false`) for a measured build inside codelaw — CSS 74.4% → 88.6%, SCSS 80.7% → 86.0%, YAML 89.9% → 97.3%, Markdown 93.3% → 97.3% on this corpus — or refuse, and W-141 closes with the engine gaps named in SR-FORMATTER.

## Built 2026-11-05 (Claude Code, Opus 5.5) — Oxc behind the formatter

`crates/codelaw-format/src/oxc.rs` formats CSS and SCSS with `oxc_formatter_css`, YAML with `oxc_formatter_yaml` and Markdown with `oxc_formatter_markdown`, at oxc-project/oxc git `7b933d5` ([SR-LAW-11](../../records/0013_LAW-11-dependencies.md) D-52: rev, 74-package tree and ledger lines written; `oxc_allocator` patched to the same rev so the crates.io parsers share one copy — named in [SR-PACKAGES](../../records/0107_packages.md) decision 4). Markdown's fenced blocks and front matter go to codelaw's own engine for their language through Oxc's dispatcher; the verifier's parse check uses Oxc's parsers; `prettier-ignore` is native to all three. Biome's CSS crates, `pretty_yaml` (D-07) and dprint (D-08) left the build. `oxc_formatter` for JS/TS is outside D-52 and was not built.

| language | files | before (Biome / pretty_yaml / dprint) | Oxc | oxfmt 0.75.0 |
|---|--:|--:|--:|--:|
| CSS | 79 | 74.4 % (1 skipped) | **88.6 %** | 88.6 % |
| SCSS | 150 | 80.7 % | **86.0 %** | 86.0 % |
| YAML | 150 | 89.9 % (1 skipped) | **97.3 %** | 97.3 % |
| Markdown | 150 | 93.3 % | **94.7 %** | 97.3 % |
| HTML (`<style>` via CSS) | 147 | 68.7 % | 69.4 % | — |

Markdown's gap to oxfmt is fenced JS: Biome keeps strict-mode-invalid examples (octal literals, `eval` as a parameter) as written where Prettier formats them; oxfmt uses Oxc's JS formatter. SCSS's identical lines dip (98.0 → 97.4 %) from one file, Bootstrap's `_utilities.scss` (Prettier's extra indent on nested maps after a comment).

**Size and speed** (Apple M4, loaded by parallel agents; binaries run alternately): binary 37,958,944 → 35,666,544 bytes; `format --check` on the 529 CSS/SCSS/Markdown/YAML files 118 → 49 ms (0.43 → 0.19 s on one thread); `codelaw bench` p95 — `format_warm_10k` 917 / 743 → 715 / 708 ms (budget 2,379), `startup` 2.2 → 2.2 ms over three 30-run sessions (budget 3; one 10-run session read 3.6 ms under load, not reproduced), `fix_one_file` 246 / 216 → 218 / 210 ms (unset). **Layout check:** `codelaw test` on `css`, `scss`, `html`, `security-html`, `css-modules`, `tailwind`, `data`, `spell`, `deps` — 0 failed before and after (`css` advisory warnings 3 → 2).

**Left:** DoD 3 (≥ 98 % or the difference named in SR-FORMATTER) — what remains is in Biome (JS/TS), markup_fmt (HTML) and Oxc (CSS/SCSS/Markdown); W-148 step 4 waits on W-139.

## Blockers

None.

## Record affected

[SR-FORMATTER](../../records/0105_formatter.md) decisions 3 and 5.

## Hazards

- A change to output reformats every consumer's files on upgrade — CHANGELOG it as a formatting change.
- The layout check (SR-FORMATTER decision 3) compares token streams: none of these changes may alter tokens.
- Corpus files keep their licences (Bootstrap MIT, Bulma MIT, Primer MIT, starter-workflows MIT, MDN learning-area CC0/MIT); name them, do not vendor large trees.

## Reference

[Proposal](../proposals/rule-parity.md) §7 · `work/proposals/rule-parity/method/` · W-07 (the formatter) · B-045 · [W-148](W-141-formatter-parity.md) (Oxc's formatters measured on this corpus: CSS 88.6%, Markdown 97.3%)

## Merged in: W-148

<a id="merged-in-w-148"></a>

**W-148 — From Oxlint: Oxc's formatters, 18 rules, 38 bug rules**, merged into W-141 on 2026-10-30; its file is archived at [`archive/open/W-148-oxc-formatters-and-oxlint-rules.md`](../../archive/open/W-148-oxc-formatters-and-oxlint-rules.md). As it stood:

### At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build `oxc_formatter_css`, `_markdown`, `_yaml` (and `oxc_formatter` for JS/TS) behind codelaw's formatter on a branch; rerun W-141's corpus; measure size, speed (L8) and the layout check | a measured case, not a claim | ✅ 2026-11-05 — CSS, SCSS, YAML, Markdown adopted; JS/TS not (outside D-52) |
| 2 | Write the L11 ratification entry (crates, pinned rev, licences, dependency tree, binary size) and put it to Arpit | his sign-off, or a no | ✅ ratified 2026-11-05 (D-52); rev and tree written at the build |
| 3 | The 18 Oxlint-only rules (`oxlint.csv`, class `port`) built in W-139 | bugs no other tool catches | ✅ added to W-139's list 2026-10-30 |
| 4 | The 38 tranche B/C ports Oxlint ranks `correctness` run on the corpus once W-139 has them; those with no false positives proposed for `recommended` | more bugs caught by default, safely | 🟡 waits on W-139 |

**Arpit, 2026-10-30 (voice):** *"Put it in backlog. Everything else, put it under a work. File it under a work item."* (the backlog part is `codelaw migrate oxlint|oxfmt`, B-066) · then *"File it."*

**Researched 2026-10-30 (Cowork, Opus 5.5):** *"Research auxlint and what can be used from that into code law."* — no tool by that name; read as Oxlint (Oxc, MIT). [Proposal §12](../proposals/rule-parity.md).

**Model:** Opus (Claude Code): a formatter swap measured against a corpus, and a dependency case under L11.

### Goal

**Step 1's starting numbers.** oxfmt 0.75.0 run on W-141's corpus with `printWidth: 80`, against Prettier 3.9.10's output (byte-identical files):

| language | files | oxfmt | codelaw 0.4.0 |
|---|--:|--:|--:|
| CSS | 79 | 88.6% | 41.8% |
| Markdown | 150 | 97.3% | 73.3% |
| YAML | 150 | 97.3% | 88.7% |
| SCSS | 150 | 88.0% | 82.7% |
| JS | 150 | 96.0% | 94.7% |
| TS | 150 | 94.0% | 94.0% |

- **Native Rust, MIT:** CSS/SCSS/Less, Markdown, YAML, JSON, TOML, GraphQL and JS/TS (`apps/oxfmt/src/core/support.rs`, `FormatStrategy::Native`). Their direct dependencies are small (`oxc-css-parser`, `oxc-markdown-parser`, `oxc-yaml-parser`, `oxc_formatter_core`, `oxc_allocator`).
- **Not on crates.io** (`publish = false`): a git-rev dependency, as codelaw pins Biome's today.
- **Not usable:** HTML, Vue, Svelte, Astro — oxfmt hands them to Prettier (JavaScript); oxfmt's 100% on HTML is Prettier's own output. W-143 cannot take them.
- **JS/TS gain is small** (+1.3 points JS): the case is CSS and Markdown; decide JS/TS separately.
- Oxc's own Prettier-fixture numbers (`apps/oxfmt/conformance/snapshots/conformance.snap.md`): CSS 221/221, Markdown 273/283, YAML 301/302, SCSS 203/217, GraphQL 712/712.

**Step 3's rules** (11 in `js`/`node`, 7 in `react`): array methods on `arguments`; `charAt` compared with a multi-character string; `matchAll`/`replaceAll` given a regex without `g`; `if`/`else` sharing code; erasing ops (`x * 0`); `async` Express handlers; spreads inside `.map`; `this` in an exported function; out-of-range radix/precision; callbacks on arrays of empty slots; `cloneElement`; `React.Children`; `only-export-components` (Fast Refresh); class components; JSX, array and object literals passed as props.

**Step 4's list:** `oxlint.csv` rows whose `oxlint_category` is `correctness` and whose `same-as` source row is tranche B or C — 38 (`js/no-eval`, `js/no-thenable`, `react/no-unsafe`, `test/no-conditional-tests` …).

### Definition of done

| DoD | what | state |
|---|---|---|
| 1 · measured | corpus table per language for codelaw-with-Oxc; L8 budgets and the layout check unchanged or explained; binary size before/after | ✅ § Built 2026-11-05 |
| 2 · ruled | an L11 entry (D-nn) ratified or refused by Arpit; if ratified, SR-FORMATTER names which languages use which formatter and W-141 continues from there | ✅ D-52; SR-FORMATTER decision 2 |
| 3 · rules | the 18 in W-139's DoD 1 | ✅ |
| 4 · defaults | the 38 corpus-run; promotions proposed with their finding counts | ⏳ |

### Blockers

None for steps 1–2. Step 4 🟡 W-139.

### Record affected

[SR-LAW-11](../../records/0013_LAW-11-dependencies.md) (a ratified dependency) · [SR-FORMATTER](../../records/0105_formatter.md) · SR-RULE-PACKS (presets, step 4).

### Hazards

- **Output churn:** a formatter swap reformats consumers' files on upgrade; CHANGELOG it, and keep each language's switch separate.
- **The layout check** (SR-FORMATTER decision 3) must still see the same tokens.
- **A git rev on an unpublished crate** moves only when codelaw moves it; pin it, and say who bumps it.
- **Rules:** reimplement from behaviour; Oxlint is MIT, but codelaw writes its own cases and docs.

### Reference

[W-141](W-141-formatter-parity.md) · [W-139](W-139-rule-parity-build.md) · [W-143](W-143-vue-svelte-astro-graphql.md) · `work/proposals/rule-parity/oxlint.csv` · `method/oxlint-rules.jsonl` · [oxc-project/oxc](https://github.com/oxc-project/oxc) (MIT)
