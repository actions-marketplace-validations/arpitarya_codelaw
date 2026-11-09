---
type: Handoff
name: W-194
description: "Arpit, 2026-11-09 (voice docket): the Pylint rows with no blocker are built — the 15 docparams rows and two links to existing rules."
item: W-194
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-194 — The Pylint rows with no blocker: docparams and two links

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Build the 15 docparams rows on the docstring parser | Pylint's docstring-parameter checks | ✅ 2026-11-09 — `py_pylint_doc.rs` (its own Sphinx/Google/NumPy reading), `preview`, W-139-pydoc, slice 37 |
| 2 | Link `preferred-module` and `consider-using-f-string` to the rules that do their job | no duplicate rules | ✅ 2026-11-09 |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* Unblocked rows go to build items: *"Only the ones that are blocked."* Filed on *"file it."*

**Model:** Sonnet (Claude Code): the parser exists (`py_ruff_doc.rs`).

## Goal

Pylint docparams: `differing-param-doc`, `differing-type-doc`, `missing-any-param-doc`, `missing-param-doc`, `missing-raises-doc`, `missing-return-doc`, `missing-return-type-doc`, `missing-type-doc`, `missing-yield-doc`, `missing-yield-type-doc`, `multiple-constructor-doc`, `redundant-returns-doc`, `redundant-yields-doc`, `useless-param-doc`, `useless-type-doc`. Links: `pylint/preferred-module` on the banned-import rule's `ported_from:`; `pylint/consider-using-f-string` on `py/printf-string-formatting` and `py/f-string`.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · rules | the 15 with cases | ✅ options `accept_no_*_doc` keep Pylint's `true`; `docstring-min-length` and a custom `no-docstring-rgx` are not options; `missing-raises-doc` does not infer a raised class from a call's return value |
| 2 · links | the two `ported_from:` lines | ✅ `use-instead`, `printf-string-formatting`, `f-string` |

## Blockers

None.

## Record affected

no SR affected.

## Hazards

- docparams is an opt-in extension upstream, with `accept-no-*-doc` options: keep it opt-in.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) (*Slice 35*) · `work/proposals/rule-parity/pylint.csv`
