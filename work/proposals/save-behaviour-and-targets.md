---
type: Proposal
description: "W-163's research (2026-11-05): which automatic fixes wait for `codelaw fix` instead of running on save or after each agent edit, and target versions as an input to fixes — both configurable per rule and per project, each rule with a default."
item: W-163
date: 2026-11-05
author: "Cowork (Opus 5.5)"
---

# Fixes that wait, and fixes that need a target

**The takeaway.**

- **Save behaviour:** one rule key, `on_edit`, with a default per rule. A
  project overrides it the way it already lowers a fix (SR-CONFIG decision 11d).
  14 `auto` rules default to `false` today, and 10 more take `false` as they
  move up to `auto` (§1.3). Every multi-file fix is forced `false`.
- **Targets:** one `[targets]` table, with `python` and `browsers`, overridable
  per scope and per rule. Python comes from `requires-python`. Browsers come
  only as explicit minimums. With no target, a target-dependent fix is
  `suggest` (as ruled).
- **What the targets unlock:** 21 rules, not 12. Ruff's pyupgrade ports
  (W-139) add nine Python rules with a version floor, two of them `auto` today.
- **Two calls for the decider** (§3):
  1. codelaw writes its own small browser-support table, since caniuse and MDN
     are both refused under L2.
  2. Two `auto` Python rules are safe only from 3.8 and 3.9; under the ruling,
     they become target-gated.

## 1 · `on_edit` — which fixes wait

### 1.1 · The surfaces

| surface | applies `auto` today | with `on_edit: false` |
|---|---|---|
| save in the editor (LSP `source.fixAll.codelaw`) | yes | **no** |
| the agent's per-edit hook (`codelaw hook`) | yes | **no**: the agent may add an import one edit before its use |
| `codelaw fix`, a pre-commit run of it, CI's `fix --check` | yes | yes |
| the editor's quick fix on the finding | offered | offered |

`suggest` fixes are unaffected: no surface applies them unasked (L4
decision 4).

### 1.2 · The keys

- **Rule frontmatter:** `on_edit: false`. The default is `true`, so no rule
  file changes unless it opts out. `fix_reason:` says why, in one line.
- **Project, per rule:** `"py/unused-import" = { on_edit = true }` in
  `[rules]`, a `[scope.<name>]` or a `[profile.<name>.rules]`, the same three
  places decision 11d's `fix` lowering lives.
- **Project, all rules:** `[fix] on_edit = false`, for teams that want save
  never to edit. It sits below the per-rule entries.
- **Precedence**, lowest first: rule default < `[fix] on_edit` < `[rules]`
  entry < scope < profile. `config --print` and `rules --why` show the result
  and the line that set it, as they do for `fix`.
- **Both directions are allowed.** Unlike a rung (decision 16, lower only),
  turning `on_edit` back on is safe: the fix is still proven
  behaviour-preserving, only its timing changes.
- **Forced `false`, not overridable:** any fix that edits more than one file
  (W-161 §2.2b).

### 1.3 · The defaults — which rules start at `false`

A rule defaults to `false` when its edit **deletes something a person or agent
adds on purpose while still working**:

| kind | `auto` today → `false` | `false` when it moves up to `auto` (W-156, W-162) |
|---|---|---|
| unused bindings | `js/no-unused-labels`, `js/regexp-no-unused-capturing-group`, `py/unused-annotation`, `py/format-unused-named-argument`, `py/format-unused-positional-argument`, `py/percent-format-unused-named-argument` | `js/no-unused-imports`, `py/unused-import`, `py/unused-variable`, `js/no-unused-vars`, `js/no-unused-private-class-members`, `py/redefined-while-unused` |
| unreachable code | — | `js/no-unreachable`, `py/unreachable` |
| test focus | — | `js/no-focused-tests` |
| debugging aids | `js/no-debugger` | — |
| placeholders being filled in | `css/no-empty-block`, `js/no-empty-static-block`, `py/empty-type-checking-block`, `py/f-string-missing-placeholders` (an `f"…"` before its `{x}` is typed), `css/comment-no-empty`, `scss/double-slash-comment-empty`, `py/empty-comment` | `js/no-lone-blocks` |

**Today the most destructive fixes are not `auto` at all.** Unused imports,
unreachable code and focused tests are `suggest`, so no save applies them yet.
`on_edit` matters as W-156 and W-162 move them up: **a move to `auto` in these
kinds sets `on_edit: false` in the same change.** That way the climb up the
ladder never makes save destructive.

**Every other `auto` rule stays `true`.** Quotes, sorting and redundant
parentheses change text, not intent. Stub-file rules (`*.pyi`) stay `true`:
stubs aren't drafts.

### 1.4 · Prior art

- **Biome:** a per-rule `fix` option (`none`, `safe`, `unsafe`). Safe fixes
  apply on save; unsafe ones never do ([Biome: linter](https://www.biomejs.dev/linter)).
- **vscode-eslint:** `eslint.codeActionsOnSave.rules` chooses which rules'
  fixes run on save ([vscode-eslint #1364](https://github.com/microsoft/vscode-eslint/pull/1364/files)).
- **codelaw's difference:** the default is per rule and written in the rule,
  so a new user gets sane saves without configuring anything.

## 2 · Targets

### 2.1 · The rules a target unlocks

| target | rule | safe from | today |
|---|---|---|---|
| python | `py/zip-without-explicit-strict` | 3.10 | suggest |
| python | `py/batched-without-explicit-strict` | 3.13 | suggest |
| python | `py/map-without-explicit-strict` | 3.14 | suggest |
| python | `py/non-pep585-annotation` (UP006) | 3.9 | suggest |
| python | `py/non-pep604-annotation-union` (UP007) | 3.10 | suggest |
| python | `py/non-pep604-annotation-optional` (UP045) | 3.10 | suggest |
| python | `py/datetime-timezone-utc` (UP017) | 3.11 | suggest |
| python | `py/timeout-error-alias` (UP041) | 3.11 | suggest |
| python | `py/non-pep695-type-alias` (UP040) | 3.12 | suggest |
| python | `py/unnecessary-default-type-args` (UP043) | 3.13 | suggest |
| python | `py/lru-cache-with-maxsize-none` (UP033) | 3.9 | **auto**: see §3.2 |
| python | `py/lru-cache-without-parameters` (UP011) | 3.8 | **auto**: see §3.2 |
| browsers | `css/property-no-vendor-prefix`, `value-no-vendor-prefix`, `selector-no-vendor-prefix`, `at-rule-no-vendor-prefix`, `media-feature-name-no-vendor-prefix` | per feature (§2.4) | suggest / manual |
| browsers | `css/color-function-notation`, `alpha-value-notation`, `color-function-alias-notation` | CSS Color 4 support | suggest |
| browsers | `css/property-no-deprecated` (the legacy aliases) | per property | suggest |

`py/sys-version-slice3` is not on this list. Its fix changes the output
string, so a target can't make it `auto`.

### 2.2 · Where a target comes from

Python's sources, highest first:

1. a rule entry's `targets.python`;
2. a scope's `targets.python` (a `legacy/` folder can say `"3.8"`);
3. `[targets] python = "3.10"`;
4. the **lower bound** of `project.requires-python` in the nearest
   `pyproject.toml` above the file, so a monorepo gets each package's own;
5. otherwise **unknown**.

Reading the lower bound is a small PEP 440 job: `>=3.10`, `~=3.10`, `>3.9`
and `==3.11.*` all give one, while `<4` or `*` gives none. The parser that
`[deps.runtime]` already uses to read `requires-python` (`codelaw-config`
`RUNTIME_SOURCES_PYTHON`) is the place to share it.

`.python-version` is **not** a source. It names the interpreter a developer
runs, not the oldest one the code supports, so for a library it overstates
the floor. `codelaw migrate` copies Ruff's `target-version` into
`[targets] python`, which is where a Ruff user's setting goes.

**Browsers:** `[targets] browsers = ["chrome >= 111", "firefox >= 113", "safari >= 16.4"]`,
overridable per scope and per rule the same way.

- A `browserslist` in `package.json`, or a `.browserslistrc`, counts **only
  when every query is `<browser> >= <version>`**.
- Any other query (`defaults`, `last 2 versions`, `> 0.5%`, `not dead`) leaves
  the target unknown, and `config --print` says why.
- The browser names are a closed set: `chrome`, `edge`, `firefox`, `safari`,
  `ios_saf` and `samsung`.

### 2.3 · With no target

**The fix is `suggest`**, never `auto` (ruled). An `auto` fix must not change
behaviour (L4 decision 3), and a guessed target breaks that on an older setup.

- The finding's note names the one line that would make it `auto`.
- Rule files gain one key, `fix_target: { python: "3.10" }`, or a browsers
  feature id for the CSS rules. With it, the rule's `auto_when` holds only when
  the resolved target meets the key.

### 2.4 · The browser data

A target says *which browsers*. The CSS rules also need *since which version
each feature is unprefixed*. That is data, and both public sources are closed
to codelaw:

- caniuse is CC-BY-4.0;
- MDN's `browser-compat-data` is CC0, which [SR-RULE-PACKS](../../records/0101_rule-packs.md)
  already refuses, because it isn't OSI-approved.

The way through is a table codelaw writes itself:
`packs/css/data/support.json`, one row per feature the nine rules rewrite, with
the first unprefixed version per browser and each row's source cited (the
browser's release notes or the spec). It carries the NOTICE the data-table rule
asks for. Its size is set by what the rules already know is prefixed
(`packs/css/data/css/prefixed.json`): **about 250 rows** (141 properties, 82
values, 15 selectors, 8 at-rules), plus a handful for Color 4 and the legacy
aliases.
**This is §3.1's call.**

## 3 · Two calls for the decider

**Ruled 2026-11-05:** both as recommended; the table ships as its own data package on npm and PyPI ([L6](../../records/0008_LAW-6-python-and-node-packages.md) decision 15), with a weekly CI check. Built by [W-166](../../archive/open/W-166-save-behaviour-and-targets-build.md).

### 3.1 · The browser-support table

**Recommended: write it.** It's small, it's codelaw's own, and every row is
cited. It lets nine CSS rules fix automatically for any project that states
its targets.

**The alternative:** no table. The nine rules stay `suggest` forever, and
browser targets are only read for the colour notation rules, whose test is
one feature.

**The cost of writing it:** about 250 cited rows, kept current as browsers
ship. To keep it small, it can start with only the features the pinned corpus
actually finds (W-138's corpus). Every other prefixed feature stays `suggest`
until it has a row.

### 3.2 · The two `auto` Python rules with a floor

`py/lru-cache-with-maxsize-none` (`functools.cache`, Python 3.9) and
`py/lru-cache-without-parameters` (Python 3.8) are `auto` today. On an older
Python the edit raises an error.

Under the ruling ("no target → `suggest`") they **become target-gated**:
`auto` when the target is at least the floor, `suggest` otherwise. That takes
two rules off the `auto` count for projects that state no target.

**Recommended: apply the ruling as it stands.** The alternative is a codelaw
floor of "Python ≥ 3.9 is assumed", which is what Ruff does (default 3.9 since
v0.8, [Ruff v0.8](https://astral.sh/blog/ruff-v0.8.0)). That is a guess about
the user's runtime, and the ruling chose not to guess.

## 4 · Out of scope, named

- **JavaScript and Node targets** (an `engines.node` or ES year for fixes like
  `.at()` or `Object.hasOwn`). The same `[targets]` table extends to them.
  Today only `js/prefer-import-meta-properties` (Node 20.11) is `suggest`
  for a version reason. The ten `prefer-global-*` rules are `suggest` for a
  bundler reason, so a target wouldn't move them. Not worth a ruling yet.
- **`.python-version` and `.tool-versions`** as Python sources: refused above
  (§2.2).

## 5 · The build, once ruled

1. The `on_edit` key, its config, the LSP and hook surfaces, and the 14
   rules' defaults (§1.3).
2. `[targets]`, its sources and precedence, and `config --print` /
   `rules --why`.
3. The `fix_target` key on the twelve Python rules.
4. The browser table and the CSS rules, if §3.1 is ruled yes.

**Records:** SR-LAW-4 decision 12 (what the hook applies), SR-CONFIG (`[fix]`,
`[targets]`, rule entries), SR-RULE-FORMAT (`on_edit`, `fix_target`),
SR-RULE-PACKS (the data table) and SR-AGENT-SURFACES.

## Reference

[W-163](../open/W-163-save-behaviour-and-targets.md) ·
[W-156](../../archive/open/W-156-fix-ladder.md) (open questions 1–2, as ruled 2026-11-05) ·
[SR-LAW-4](../../records/0006_LAW-4-every-rule-has-a-fix.md) decisions 3, 12, 16 ·
[SR-CONFIG](../../records/0110_config.md) 11d ·
[`fix-ladder.md`](fix-ladder.md) §open questions
