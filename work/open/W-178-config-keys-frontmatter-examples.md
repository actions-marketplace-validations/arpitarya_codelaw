---
type: Handoff
name: W-178
description: "Arpit, 2026-11-09 (voice docket): build key-name-casing, require-frontmatter-fields and check-examples — W-139 rows first sent to the backlog, then pulled back."
item: W-178
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-178 — Key casing in config files, Markdown frontmatter, and linting `@example` code

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build `key-name-casing` in the `data` pack (JSON, YAML, TOML keys), off by default | one casing style for config keys | ⏳ |
| 2 | Build `require-frontmatter-fields` in the `md` pack | Markdown files carry the fields a project lists | ⏳ |
| 3 | Teach the engine to lint the code in a doc comment's `@example` as its own region | examples in docs can be checked | ⏳ |
| 4 | Build `check-examples` on that region | stale or broken examples are reported | ⏳ |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* On these three: *"Okay, pull it back from backlog. I want these built."* — confirmed as one item, `check-examples` with its engine work: *"Yes."* Filed on *"file it."*

**Model:** Opus (Claude Code): step 3 is engine design (an embedded region inside a comment); the two rules are routine.

## Goal

The three rows ship. `key-name-casing` and `require-frontmatter-fields` check nothing until a project names its casing or its fields, so they read their options the way W-176 settles.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rules | `data/key-name-casing` (off by default), `md/require-frontmatter-fields`, `js/jsdoc-check-examples`, each with cases and a fix class (L4) | ⏳ |
| 2 · engine | an `@example` block linted as code of its language, findings mapped back to the comment's lines | ⏳ |
| 3 · docs | rule pages; the parity ledger marks the three built | ⏳ |

## Blockers

None. The two option rules use W-176's option shape when it lands; build them after it or land the shape with them.

## Record affected

SR-RULE-PACKS (the `data` and `md` packs) and SR-ARCHITECTURE (a region embedded in a comment) — the builder names the decisions.

## Hazards

- eslint-plugin-jsdoc's own `check-examples` says it does not work on ESLint 8+; read its behaviour, not its mechanism.
- An example is often a fragment: decide what is reported when it does not parse, and say so on the rule page.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (the 2026-11-09 inbox row) · `work/proposals/rule-parity/eslint-plugin-unicorn.csv`, `eslint-plugin-jsdoc.csv`
