---
type: Handoff
name: W-176
description: "Arpit, 2026-11-09 (voice docket): split W-139's options-only js and node rows into their own item — rules that check nothing until a project sets their option, shipped silent by default and not counted against W-139's parity."
item: W-176
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-18
updated_by: "Claude Code (agent session 9ab51fd9): built and closed"
ball: agent
---

# W-176 — Rules that work only once a project configures them

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Settle one way for a rule to read its list or pattern from the project's settings | the same shape for all 19 rules | ✅ [SR-RULE-FORMAT](../../records/0112_rule-format.md) decision 2e |
| 2 | Build each rule so it reports nothing until its option is set | the rules ship, quiet by default | ✅ 18 rules, 2026-11-18 (*Built*) |
| 3 | Mark the 19 rows with their own class in the parity ledger | W-139 no longer counts them as missing | ✅ 18 rows, class `options-only` |

**Arpit (voice, Cowork docket), 2026-11-09:** shown three of them (`id-match` with a camelCase pattern, `id-denylist` with `data`, `temp`, `e`, `no-restricted-syntax` with `for...in`), asked whether they should ship as configure-to-use rules: *"Yes."* Then: *"For the previous item, create a separate work item."* — confirmed *"Yes."*; filed on *"file it."*

**Model:** Opus (Claude Code): step 1 is a design call on how `[rules]` options reach a rule; the 19 rules after it are routine.

## Goal

Every upstream rule that does nothing in its default configuration ships in codelaw as a rule a project switches on by giving it its option. With no option it is silent: no finding, no warning about the missing option. It is its own class, so it never counts as a missing port in W-139.

## The rows

From W-139 (*Strand W-139-js, second pass*: "does nothing in its default configuration", and the eslint-plugin-jsdoc rows left at 65.4.2):

| pack | rows |
|---|---|
| `js` | `no-implicit-globals`, `import-no-restricted-paths`, `comment-regex`, `consistent-function-style`, `file-header`, `id-denylist`, `id-match`, `id-match-unicorn`, `import-prefer-namespace-import`, `jsdoc-match-name`, `jsdoc-no-missing-syntax`, `jsdoc-no-restricted-syntax`, `jsdoc-require-tags`, `jsdoc-text-escaping`, `name-replacements`, `no-restricted-syntax`, `string-content` |
| `node` | `no-restricted-import`, `no-restricted-require` |

Not here: `js/key-name-casing` and `js/require-frontmatter-fields` (W-139's own inbox row puts them in the `data` and `md` packs). `no-implicit-globals` is in this class because ESLint 10 reads `.js` as a module, where it never fires; check that this reading still holds before building it. `name-replacements` also needs its replacement table and scope-aware renames.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · shape | how a rule reads its option, written into the owning record | ✅ SR-RULE-FORMAT decision 2e |
| 2 · rules | the 19, each with valid and invalid cases (with the option set) and a case showing it is silent without it; its fix class (L4) | ✅ 18 (`name-replacements` is not one of them: *Built*) |
| 3 · ledger | the 19 rows carry their class in `work/proposals/rule-parity/*.csv`; W-139's DoD 1 count leaves them out | ✅ 18 rows, `classify.py` `RULED` and the proposal's §3 |
| 4 · docs | each rule page names its option and shows a config example | ✅ |

## Built (2026-11-18)

**The shape** (SR-RULE-FORMAT decision 2e): the option is declared with an empty default and a type from decision 2's list (a list of objects is an `object` keyed by a name the project chooses); the rule names its options in `NativeRule::needs_options`, and the engine asks it about no node, and builds no semantic model for it, while none holds a value; silent means no finding and no warning that the option is missing (a value that cannot be used is reported once, on the file's first node); `preview`, in no preset; examples carry `options=`, and each `.cases.md` has a case with no option that expects `(none)`; corpus count 0.

**The 18 rules**, `crates/codelaw-native/src/options_only/`: `js/id-denylist`, `js/id-match`, `js/id-match-unicorn`, `js/no-restricted-syntax` (ast-grep patterns and tree-sitter kinds stand for ESLint's ESTree selectors), `js/no-implicit-globals` (`script`: the files that are classic scripts), `js/comment-regex`, `js/file-header`, `js/consistent-function-style`, `js/import-prefer-namespace-import`, `js/import-no-restricted-paths` (`zones`, named), `js/string-content`, `js/jsdoc-match-name`, `js/jsdoc-no-missing-syntax`, `js/jsdoc-no-restricted-syntax`, `js/jsdoc-require-tags`, `js/jsdoc-text-escaping`, `node/no-restricted-import`, `node/no-restricted-require`. Each page says what it reads of upstream and what it does not.

**Not 19:** `js/name-replacements` is in the item's list, but upstream runs it on a built-in table of about a hundred names, so it does not do nothing by default; W-139 slice 36 built it that way (801 findings in the corpus), and its ledger row stays a built port.

**Checks:** `codelaw test packs/js` and `packs/node` 0 failed; `cargo test -p codelaw-native options_only` and `-p codelaw-cli --test options_only`; `cargo clippy -p codelaw-native -p codelaw-engine -- -D warnings`.

## Blockers

None.

## Record affected

SR-RULE-FORMAT (how a rule reads its `[rules]` options) and SR-RULE-PACKS (the class and which presets may hold these rules). The builder names the decisions.

## Hazards

- W-139's js strand holds `packs/js` files while it is open: claim only these rules' files.
- A rule that reports "option missing" is not silent. Upstream jsdoc rules do that; codelaw's do not.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) — its 2026-11-09 ruling, *Strand W-139-js, second pass*, and the eslint-plugin-jsdoc table under *Needs native — W-139-js*.
