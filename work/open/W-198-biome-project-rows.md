---
type: Handoff
name: W-198
description: "Four of Biome's rows need a view across files that plain CSS and JSX do not have yet; W-196 classed them here."
item: W-198
filed: 2026-11-18
filed_by: "Claude Code (Sonnet 5.5), from W-196"
updated: 2026-11-18
updated_by: "Claude Code (Sonnet 5.5)"
ball: agent
---

# W-198 — Biome's four project-wide rows

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Read what each row asks of the project graph and choose the shape (a `graph:` form, or a native rule on the module graph) | one design for the four | ⏳ |
| 2 | Build the four, each with cases that span files | the rows done | ⏳ |

**Model:** Sonnet (Claude Code); step 1 needs the record (SR-MODULE-GRAPH, SR-PROJECT-GRAPH) read first.

## Goal

Biome 2.6.4 rows that [W-196](../../archive/open/W-196-biome-tail.md) found need more than one file, with the proposed ids of `work/proposals/rule-parity/biome.csv`:

| row | what it checks | proposed id |
|---|---|---|
| `correctness/noPrivateImports` | an import of an export whose JSDoc says `@private` or `@package`, from a module outside the allowed folders (index files, `node_modules`, dynamic `import()` and `require()` are left out) | `js/no-private-imports` |
| `nursery/noUndeclaredClasses` | a class used in JSX or HTML that no stylesheet reaching the file declares | `js/no-undeclared-classes` |
| `nursery/noUnusedClasses` | a class a stylesheet declares that no JSX or HTML file uses | `css/no-unused-classes` |
| `nursery/noUndeclaredCustomProperties` | a `var(--x)` with no declaration or `@property` reaching it | `react/no-undeclared-custom-properties` |

`css-modules/no-unused-class` and `css-modules/no-undefined-class` already do the CSS Modules half through a `graph:` form; these four are the same questions for global stylesheets and for JSDoc visibility.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · design | the graph facts the four read, in the record that owns the graph | ⏳ |
| 2 · rules | the four built, `preview`, with multi-file cases | ⏳ |
| 3 · ledger | `biome.csv`'s four `planned` rows read built | ⏳ |

## Blockers

None.

## Record affected

SR-MODULE-GRAPH or SR-PROJECT-GRAPH (the new graph facts); SR-RULE-PACKS decision 18.

## Hazards

- A class reaches a file by `<link>`, `@import`, a JS `import "./x.css"` or a bundler convention; start from what the project's own files say and leave the rest alone, as `css-modules/*` does.

## Reference

[W-196](../../archive/open/W-196-biome-tail.md) · `work/proposals/rule-parity/biome.csv`
