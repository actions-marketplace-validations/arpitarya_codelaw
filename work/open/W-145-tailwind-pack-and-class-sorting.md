---
type: Handoff
name: W-145
description: "Arpit, 2026-10-30 (voice): a Tailwind pack — its bug-catching rules and the class-order table as data — and Tailwind class sorting in codelaw's formatter, on when the project uses Tailwind."
item: W-145
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-10-30
updated_by: "Claude Code (Opus 5.5)"
ball: arpit
---

# W-145 — The Tailwind pack and class sorting in the formatter

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Amend L6: a `tailwind` pack | one package to install | ⏳ after W-139's packs land (same lines) |
| 2 | The class-order table as pack data, matching `prettier-plugin-tailwindcss`'s order | no churn for teams switching | 🔴 L11: Tailwind not ratified (Blockers) |
| 3 | The formatter sorts `class`/`className` (and `clsx`/`cn`/`tw` calls, configurable) when the pack is installed and the project uses Tailwind | sorting with no extra tool | ⏳ |
| 4 | The lint rules W-144 picked: the 8 `tailwind` rows in the two Tailwind CSVs (conflicting, unknown, concatenated classes …); `tailwind/recommended` on when the pack is installed | bugs caught | ⏳ ruled 2026-10-30 |

**Arpit, 2026-10-30 (voice):** *"I want the class sorting in the formatter."* · on a linter package and a formatter package: told packs hold data, never code, and the formatter lives in the engine — *"But will the package size increase significantly?"* → no, tens to a few hundred kilobytes (estimate) → *"Okay. Then let's go for it."*

**Model:** Opus (Claude Code).

## Goal

- **One pack, both registries** (Tailwind classes live in JSX, HTML and Python templates): `@codelaw/rules-tailwind`, `codelaw-rules-tailwind` (PyPI name free, checked 2026-10-30).
- **Sorting is on** when the pack is installed and the project uses Tailwind (a `tailwindcss` dependency, a `tailwind.config.*`, or `@import "tailwindcss"` in a stylesheet); `[format.tailwind] sort = true | false` overrides; plain Prettier does not sort, so it is never on without Tailwind.
- **Where it sorts:** `class` in HTML, `className` in JSX/TSX, and calls named in `[format.tailwind] functions` (default `clsx`, `cn`, `cva`, `tw`).
- **The order:** Tailwind's own (layer, then property order, then variants), from a table generated once from Tailwind and committed as pack data; custom utilities from the project's CSS `@theme`/`@utility` read as data; unknown classes first, as the Prettier plugin does.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · law | L6 amended (the pack, both registries); CLAUDE.md's L6 line | ⏳ |
| 2 · table | generated from Tailwind v4 by a script in `scripts/`, its source and version recorded; size measured | 🔴 L11 (Blockers) |
| 3 · sorting | byte-identical to `prettier-plugin-tailwindcss` on a corpus of Tailwind projects (≥ 98% of files) | 🔴 L11 (Blockers) |
| 4 · names | npm first publish and PyPI pending publisher — in [W-142](W-142-framework-pack-names.md) (merged 2026-10-30) | 🔴 Arpit, before release |
| 5 · records, docs | SR-FORMATTER (the `[format.tailwind]` keys), SR-RULE-PACKS, SR-PACKAGES; CHANGELOG | ⏳ |

## Blockers

🔴 **L11 — two tools nobody has ratified (found 2026-10-30, Claude Code).** W-144's ruling is unblocked; the table and the parity check are not:

- **The table** (DoD 2) is generated *from* Tailwind: its order is not written down anywhere as data — Tailwind computes it (each utility's declarations ranked by its property-order list, then the variants' registration order, then a natural compare), and `prettier-plugin-tailwindcss` asks the project's Tailwind (`getClassOrder`) for it. So the generator must run Tailwind, and the committed table is data derived from it — both are a dependency (L11 decision 1: copied-in data is one, as D-42–D-44). Pinned for the question: `tailwindcss@4.3.3`, MIT, no dependencies, tarball SHA-256 `f1493c5bcb29a80310da63d19282d233ba259479f36561a27146cd4747d1faef`.
- **The parity check** (DoD 3, ≥ 98% byte-identical) runs `prettier-plugin-tailwindcss` beside codelaw on a corpus: `prettier-plugin-tailwindcss@0.8.1`, MIT, no dependencies (peer: `prettier`; D-38 ratifies `prettier@3.9.10` for the CI bench only), SHA-256 `c2d90bbe3f89aedc62a1a1a187b0dc649125e0f85cc9028889cb0322d55388e3`. A corpus of Tailwind projects is also needed — none is pinned yet.
- **Claude Code's auto-mode permission check refused** running the downloaded Tailwind (a Node import of its `dist/`), as code from outside the repo. Even ratified, a session needs it allowed once.

**What the decider rules** (the inbox row): ratify (1) `tailwindcss@4.3.3` — run by hand by a `scripts/` generator only when the pin moves, its output committed to `packs/tailwind/data/`, never in the engine, a package, a test or CI (as D-36); (2) `prettier-plugin-tailwindcss@0.8.1` with `prettier` — the measuring instrument only, by hand or CI-only (as D-38); (3) allow running them when Claude Code asks. Refusing (1) leaves two ways: a table codelaw writes itself from Tailwind's documentation (not "generated from Tailwind", so DoD 2 changes, and parity will be lower), or no sorting.

**Nothing is built yet.** Buildable without the table, once the pack is registered: five of the eight rules are syntactic — `no-concatenated-classes`, `no-restricted-classes`, `important-modifier-suffix`, `enforces-negative-arbitrary-values`, `no-arbitrary-value`; `no-conflicting-classes`, `no-unknown-classes` and `enforces-canonical-classname` need the table. Registering the pack (FIRST_PARTY_PACKS, L6's counts) is best done after W-139 lands its seven packs, which edit the same lines.

## Record affected

SR-LAW-6 · [SR-FORMATTER](../../records/0105_formatter.md) · SR-RULE-PACKS · SR-PACKAGES.

## Hazards

- **No code in a pack, no JavaScript at runtime:** `prettier-plugin-tailwindcss` asks the project's installed Tailwind for the order; codelaw cannot (L5; Node is allowed only for type rules). A project's JS plugins that add utilities cannot be ordered exactly — they sort as unknown; say so in the docs.
- Sorting is a formatting change: the layout check must still see the same tokens (the class string is one token; only its contents reorder — record why that is layout).

## Reference

[W-144](../../archive/open/W-144-rule-parity-round-2.md) · [W-141](W-141-formatter-parity.md) · prettier-plugin-tailwindcss (MIT)
