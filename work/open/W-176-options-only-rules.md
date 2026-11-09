---
type: Handoff
name: W-176
description: "Arpit, 2026-11-09 (voice docket): split W-139's options-only js and node rows into their own item — rules that check nothing until a project sets their option, shipped silent by default and not counted against W-139's parity."
item: W-176
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-176 — Rules that work only once a project configures them

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Settle one way for a rule to read its list or pattern from the project's settings | the same shape for all 19 rules | ⏳ |
| 2 | Build each rule so it reports nothing until its option is set | the rules ship, quiet by default | ⏳ |
| 3 | Mark the 19 rows with their own class in the parity ledger | W-139 no longer counts them as missing | ⏳ |

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
| 1 · shape | how a rule reads its option, written into the owning record | ⏳ |
| 2 · rules | the 19, each with valid and invalid cases (with the option set) and a case showing it is silent without it; its fix class (L4) | ⏳ |
| 3 · ledger | the 19 rows carry their class in `work/proposals/rule-parity/*.csv`; W-139's DoD 1 count leaves them out | ⏳ |
| 4 · docs | each rule page names its option and shows a config example | ⏳ |

## Blockers

None.

## Record affected

SR-RULE-FORMAT (how a rule reads its `[rules]` options) and SR-RULE-PACKS (the class and which presets may hold these rules). The builder names the decisions.

## Hazards

- W-139's js strand holds `packs/js` files while it is open: claim only these rules' files.
- A rule that reports "option missing" is not silent. Upstream jsdoc rules do that; codelaw's do not.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) — its 2026-11-09 ruling, *Strand W-139-js, second pass*, and the eslint-plugin-jsdoc table under *Needs native — W-139-js*.
