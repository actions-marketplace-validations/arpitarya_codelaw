---
type: Handoff
name: W-145
description: "Arpit, 2026-10-30 (voice): a Tailwind pack — its bug-catching rules — and Tailwind class sorting in codelaw's formatter, on when the project uses Tailwind; the class order comes from the project's own Tailwind through a provider, @codelaw/provider-tailwind (ruled 2026-10-30)."
item: W-145
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-10-30
updated_by: "Cowork (Opus 5.5)"
ball: agent
---

# W-145 — The Tailwind pack and class sorting in the formatter

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Amend L6: a `tailwind` pack, and **providers** — a new kind of package, npm only, on the release's version; a pack may depend on its own provider | the packages to install | ⏳ after W-139's packs land (same lines) |
| 2 | `@codelaw/provider-tailwind` builds the class-order table from the Tailwind installed in the project (its peer dependency); the engine runs it with Node and caches the table by Tailwind version and config hash | the order of the Tailwind the project really uses — v3 or v4, its theme, its plugins; no copy of Tailwind in codelaw | ⏳ ruled 2026-10-30; `tailwindcss` for its tests ratified (D-48) |
| 3 | The formatter sorts `class`/`className` (and `clsx`/`cn`/`tw` calls, configurable) when the pack is installed, the project uses Tailwind and the provider gave a table; `[providers] on_missing` (`skip` by default, or `error`) says what happens when it did not | sorting with no extra tool | ⏳ |
| 4 | The lint rules W-144 picked: the 8 `tailwind` rows in the two Tailwind CSVs (conflicting, unknown, concatenated classes …); `tailwind/recommended` on when the pack is installed | bugs caught | ⏳ ruled 2026-10-30 |

**Arpit, 2026-10-30 (voice):** *"I want the class sorting in the formatter."* · on a linter package and a formatter package: told packs hold data, never code, and the formatter lives in the engine — *"But will the package size increase significantly?"* → no, tens to a few hundred kilobytes (estimate) → *"Okay. Then let's go for it."*

**Arpit, 2026-10-30 (voice, answering the L11 inbox row):** *"We shouldn't be looking into Tailwind or Prettier. As a package and generating a copy of the list in the package. It should be a peer dependency or a dependency whosoever is going to consume it. … So it should be based on whatever is present in node modules and that way whatever version it is using it will also work."* → on its name: *"Let's call it a provider."* → *"So provider is going to be a separate package, right? So that it does not get built bundled inside the engine."* → on the pack depending on its provider or naming it in its manifest: *"Why not both?"* → on a missing provider: *"So I want it configurable. That means in TOML file, we can say whether to skip it or to throw a hard error for this use case."* → *"Default to be skip. Perfect. This sounds great."* → *"file it."* (filed by Cowork).

**The ruling, as filed:**

- **No copy of Tailwind's order in codelaw.** The table generated once from Tailwind and committed to the pack (the old DoD 2) is dropped; the inbox's question (1), `tailwindcss@4.3.3` run by a `scripts/` generator, is answered *no* in that form.
- **A provider** builds a pack's data table from a package installed in the consumer's project, not from a copy in the pack. The rules read the table like any other data table. Packs stay pure data; the provider is the only part that runs code; nothing Tailwind-specific goes in the engine.
- **The first is `@codelaw/provider-tailwind`:** its own package, npm only, on the release's version, with a **peer dependency on `tailwindcss`** — the project's own. It asks the installed Tailwind for the class order (as `prettier-plugin-tailwindcss` asks `getClassOrder`).
- **The engine** finds the provider in the project's `node_modules`, runs it with Node, and caches the table by Tailwind version and config hash, so the hook stays fast.
- **The pack names its provider both ways:** an npm dependency of `@codelaw/rules-tailwind` on `@codelaw/provider-tailwind`, pinned to the exact release version (one install gets both), and `providers = ["@codelaw/provider-tailwind"]` in `codelaw-pack.toml` (`codelaw init` installs both; the engine says what to install when it is missing). L6 is amended to allow a pack's dependency on its own provider and nothing else; the pack parity check gets an exception for that one npm dependency, which the PyPI pack does not declare (its files stay identical).
- **A missing provider is configurable** in `codelaw.toml`: `[providers] on_missing = "skip" | "error"`, **default `skip`**. `skip` leaves out the rules and the sorting that need the table, with one note naming the package to install; `error` stops the run, as a missing pack does. `[providers.tailwind] on_missing` overrides it for one provider — proposed in the call and not objected to; drop it if he would rather not have it.

**Model:** Opus (Claude Code).

## Goal

- **One pack, both registries** (Tailwind classes live in JSX, HTML and Python templates): `@codelaw/rules-tailwind`, `codelaw-rules-tailwind` (PyPI name free, checked 2026-10-30); **its provider, npm only:** `@codelaw/provider-tailwind`.
- **Sorting is on** when the pack is installed and the project uses Tailwind (a `tailwindcss` dependency, a `tailwind.config.*`, or `@import "tailwindcss"` in a stylesheet); `[format.tailwind] sort = true | false` overrides; plain Prettier does not sort, so it is never on without Tailwind.
- **Where it sorts:** `class` in HTML, `className` in JSX/TSX, and calls named in `[format.tailwind] functions` (default `clsx`, `cn`, `cva`, `tw`).
- **The order:** Tailwind's own (layer, then property order, then variants), asked of the project's installed Tailwind by the provider — so the project's version, `@theme`, `@utility` and JS plugins sort as its Tailwind sorts them; unknown classes first, as the Prettier plugin does.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · law | L6 amended: the `tailwind` pack (both registries); providers — npm only, the release's version; a pack may depend on its own provider, pinned exactly, and nothing else; Node at run time for a provider (decision 3's exception widened); the pack parity check's exception. L11: `tailwindcss` named as the provider's peer dependency. CLAUDE.md's L6 line | ⏳ |
| 2 · provider | `@codelaw/provider-tailwind` builds the table from the installed Tailwind (v4; v3 if its API allows — measured, not assumed); the engine finds it, runs it with Node and caches the table by Tailwind version and config hash; cold and warm time and size measured (L8) | ⏳ — `tailwindcss` in fixtures ratified 2026-10-30 (D-48) |
| 2a · missing provider | `[providers] on_missing` — `skip` (default) or `error`, `[providers.<name>]` overrides; `skip` prints one note naming the package; `codelaw init` installs the pack and its provider together | ⏳ |
| 3 · sorting | byte-identical to `prettier-plugin-tailwindcss` on a corpus of Tailwind projects (≥ 98% of files) | ⏳ — the instrument ratified 2026-10-30 (D-49); the corpus not yet pinned |
| 4 · names | npm first publish and PyPI pending publisher — in [W-142](W-142-framework-pack-names.md) (merged 2026-10-30); `@codelaw/provider-tailwind`'s first publish too | 🔴 Arpit, before release |
| 5 · records, docs | SR-FORMATTER (the `[format.tailwind]` keys), SR-RULE-PACKS (the manifest's `providers`), SR-PACKAGES (the provider kind), SR-CONFIG (`[providers]`), SR-ARCHITECTURE (running and caching a provider); GLOSSARY (provider, coined 2026-10-30); CHANGELOG | ⏳ |

## Blockers

✅ **L11 — ratified 2026-10-30.** **Arpit, 2026-10-30 (Cowork):** *"Approve all the three items presented in the chat for work item 145."* — (1) `tailwindcss` in fixtures only: [D-48](../../records/0013_LAW-11-dependencies.md); (2) the parity instrument: D-49; (3) Claude Code may run them when its permission check asks. What was asked:

- **`tailwindcss` in codelaw's own tests** (DoD 2): the provider is tested against a real Tailwind installed into a fixture folder only — never in a package, the engine or a consumer's repo — as D-45 does for TypeScript. Pinned when it was first asked: `tailwindcss@4.3.3`, MIT, no dependencies, tarball SHA-256 `f1493c5bcb29a80310da63d19282d233ba259479f36561a27146cd4747d1faef`. The peer dependency the provider declares is the consumer's Tailwind, ruled 2026-10-30; L11's list names it when DoD 1 is written.
- **The parity check** (DoD 3, ≥ 98% byte-identical) runs `prettier-plugin-tailwindcss` beside codelaw on a corpus: `prettier-plugin-tailwindcss@0.8.1`, MIT, no dependencies (peer: `prettier`; D-38 ratifies `prettier@3.9.10` for the CI bench only), SHA-256 `c2d90bbe3f89aedc62a1a1a187b0dc649125e0f85cc9028889cb0322d55388e3`. A corpus of Tailwind projects is also needed — none is pinned yet.
- **Claude Code's auto-mode permission check refused** running the downloaded Tailwind (a Node import of its `dist/`), as code from outside the repo. Even ratified, a session needs it allowed once.

**What the decider ruled** (the inbox row, closed 2026-10-30): ratify or refuse (1) `tailwindcss` in fixture folders only, for the provider's tests (as D-45); (2) `prettier-plugin-tailwindcss@0.8.1` with `prettier` — the measuring instrument only, by hand or CI-only (as D-38); (3) allow running them when Claude Code asks.

**Nothing is built yet.** Buildable now: the records and law text of DoD 1, 2a and 5, the provider (DoD 2), and — once the pack is registered — five of the eight rules, which are syntactic — `no-concatenated-classes`, `no-restricted-classes`, `important-modifier-suffix`, `enforces-negative-arbitrary-values`, `no-arbitrary-value`; `no-conflicting-classes`, `no-unknown-classes` and `enforces-canonical-classname` need the table. Registering the pack (FIRST_PARTY_PACKS, L6's counts) is best done after W-139 lands its seven packs, which edit the same lines.

## Record affected

SR-LAW-6 · SR-LAW-11 · [SR-FORMATTER](../../records/0105_formatter.md) · SR-RULE-PACKS · SR-PACKAGES · SR-CONFIG · SR-ARCHITECTURE.

## Hazards

- **Node at run time:** the provider runs on Node, as the type rules do — today L6 decision 3 allows that for type rules only, so DoD 1 widens it (ruled 2026-10-30). No code goes in a pack.
- **No `node_modules/tailwindcss`:** a Python project that runs Tailwind's standalone CLI, or a CI image without Node, gets `on_missing` — `skip` by default — so the PyPI pack sorts nothing there; say so in the docs.
- Sorting is a formatting change: the layout check must still see the same tokens (the class string is one token; only its contents reorder — record why that is layout).

## Reference

[W-144](../../archive/open/W-144-rule-parity-round-2.md) · [W-141](W-141-formatter-parity.md) · prettier-plugin-tailwindcss (MIT)
