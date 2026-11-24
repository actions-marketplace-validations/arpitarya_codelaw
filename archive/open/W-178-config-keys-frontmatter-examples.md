---
type: Handoff
name: W-178
description: "Arpit, 2026-11-09 (voice docket): build key-name-casing, require-frontmatter-fields and check-examples — W-139 rows first sent to the backlog, then pulled back."
item: W-178
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-19
updated_by: "Claude Code (Sonnet 5.5): built and closed"
ball: none
---

# W-178 — Key casing in config files, Markdown frontmatter, and linting `@example` code

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build `key-name-casing` in the `data` pack (JSON, YAML, TOML keys), off by default | one casing style for config keys | ✅ `data/key-name-casing` |
| 2 | Build `require-frontmatter-fields` in the `md` pack | Markdown files carry the fields a project lists | ✅ `md/require-frontmatter-fields` |
| 3 | Teach the engine to lint the code in a doc comment's `@example` as its own region | examples in docs can be checked | ✅ `[lint] jsdoc_examples`, SR-LINTER decision 3 |
| 4 | Build `check-examples` on that region | stale or broken examples are reported | ✅ `js/jsdoc-check-examples` |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On these three: *"Okay, pull it back from backlog. I want these built."* — confirmed as one item, `check-examples` with its engine work: *"Yes."* Filed on *"file it."*

**Model:** Opus (Claude Code): step 3 is engine design (an embedded region inside a comment); the two rules are routine.

## Goal

The three rows ship. `key-name-casing` and `require-frontmatter-fields` check nothing until a project names its casing or its fields, so they read their options the way W-176 settles.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rules | `data/key-name-casing` (off by default), `md/require-frontmatter-fields`, `js/jsdoc-check-examples`, each with cases and a fix class (L4) | ✅ `suggest`, `manual` (`value`), `manual` (`value`) |
| 2 · engine | an `@example` block linted as code of its language, findings mapped back to the comment's lines | ✅ a region whose ` * ` margin is a hole; a fix keeps the margin |
| 3 · docs | rule pages; the parity ledger marks the three built | ✅ pages in `docs/rules/`; `eslint-plugin-unicorn.csv` rows say `data` |

## Built (2026-11-19)

**`data/key-name-casing`** (`crates/codelaw-native/src/options_only/data_keys.rs`): `case` (`camelCase`, `PascalCase`, `snake_case`, `kebab-case`, `SCREAMING_SNAKE_CASE`) and `ignore`; the keys of a JSON object, a YAML mapping (plain and quoted, block and flow) and a TOML table, its header and each part of a dotted key; `suggest`. Silent with no `case`, by W-176's shape (SR-RULE-FORMAT decision 2e). The word split is mine, not upstream's: `userID` is `user`, `id`.

**`md/require-frontmatter-fields`** (`options_only/frontmatter.rs`): `fields`; YAML and TOML front matter; a field with no value is missing; one finding per file listing what is missing.

**The `@example` region** (`crates/codelaw-lang/src/regions.rs`, `[lint] jsdoc_examples`, off by default): the lines after the tag up to the next tag, or a fence's code in the fence's language; each line's ` * ` margin, the tag and the fence lines are same-width holes, so the region's offsets are the host's and a fix cannot reach the margin; an example that does not parse is not a region.

**`js/jsdoc-check-examples`** (`jsdoc/examples.rs`): reports an `@example` that does not parse; `ignore` (regular expressions). What the project's rules find in an example that does parse is the engine's job, through the region — ESLint's `check-examples` did both, and its README says it does not work on ESLint 8 and later.

**Decided here:** an example that does not parse is reported, not linted, and a fragment must parse on its own (the hazard in the item); the `data` pack holds the two options-only rules, so the ledger's `js/` ids became `data/key-name-casing` and `md/require-frontmatter-fields`.

**Checks:** `codelaw test packs/data` 50 rules · 482 checks · 0 failed, `packs/js` 958 · 24,484 · 0 failed; `cargo test -p codelaw-config -p codelaw-lang -p codelaw-engine -p codelaw-native -p codelaw-test` and `-p codelaw-cli --test jsdoc_examples --test options_only`; clippy `--all-targets -D warnings` on lang, config, native, engine and cli. Not run: the corpus count and `perf` (the two data rules are idle unconfigured; the region is off; `js/jsdoc-check-examples` reads a comment only when it holds `@example`).

## Blockers

None. The two option rules use W-176's option shape when it lands; build them after it or land the shape with them.

## Record affected

SR-RULE-PACKS (the `data` and `md` packs) and SR-ARCHITECTURE (a region embedded in a comment) — the builder names the decisions.

## Hazards

- eslint-plugin-jsdoc's own `check-examples` says it does not work on ESLint 8+; read its behaviour, not its mechanism.
- An example is often a fragment: decide what is reported when it does not parse, and say so on the rule page.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (the 2026-11-09 inbox row) · `work/proposals/rule-parity/eslint-plugin-unicorn.csv`, `eslint-plugin-jsdoc.csv`
