---
type: Handoff
name: W-145
description: "Arpit, 2026-10-30 (voice): a Tailwind pack — its bug-catching rules — and Tailwind class sorting in codelaw's formatter, on when the project uses Tailwind; the class order comes from the project's own Tailwind through a provider, @codelaw/provider-tailwind (ruled 2026-10-30). Built and closed 2026-11-05."
item: W-145
filed: 2026-10-30
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-05
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-145 — The Tailwind pack and class sorting in the formatter

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Amend L6: a `tailwind` pack, and **providers** — a new kind of package, npm only, on the release's version; a pack may depend on its own provider | the packages to install | ✅ L6 decisions 3, 4, 11, 12, 14; L11 D-50, D-51; CLAUDE.md's L6 line |
| 2 | `@codelaw/provider-tailwind` builds the class-order table from the Tailwind installed in the project (its peer dependency); the engine runs it with Node and caches the table by Tailwind version and config hash | the order of the Tailwind the project really uses; no copy of Tailwind in codelaw | ✅ v4; SR-ARCHITECTURE decision 28a |
| 3 | The formatter sorts `class`/`className` (and `clsx`/`cn`/`cva`/`tw` calls, configurable) when a provider gave a table; `[providers] on_missing` (`skip` by default, or `error`) | sorting with no extra tool | ✅ SR-FORMATTER decision 9; sorting byte-identical on 100 % of 2,479 files |
| 4 | The lint rules W-144 picked: the 8 `tailwind` rows; `tailwind/recommended` | bugs caught | ✅ SR-RULE-PACKS decision 19 |

**Arpit, 2026-10-30 (voice):** *"I want the class sorting in the formatter."* · on a linter package and a formatter package: told packs hold data, never code, and the formatter lives in the engine — *"But will the package size increase significantly?"* → no, tens to a few hundred kilobytes (estimate) → *"Okay. Then let's go for it."*

**Arpit, 2026-10-30 (voice, answering the L11 inbox row):** *"We shouldn't be looking into Tailwind or Prettier. As a package and generating a copy of the list in the package. It should be a peer dependency or a dependency whosoever is going to consume it. … So it should be based on whatever is present in node modules and that way whatever version it is using it will also work."* → on its name: *"Let's call it a provider."* → *"So provider is going to be a separate package, right? So that it does not get built bundled inside the engine."* → on the pack depending on its provider or naming it in its manifest: *"Why not both?"* → on a missing provider: *"So I want it configurable. That means in TOML file, we can say whether to skip it or to throw a hard error for this use case."* → *"Default to be skip. Perfect. This sounds great."* → *"file it."* (filed by Cowork).

**L11, ratified 2026-10-30:** *"Approve all the three items presented in the chat for work item 145."* — D-48 (`tailwindcss@4.3.3` in fixtures), D-49 (`prettier-plugin-tailwindcss@0.8.1` with `prettier@3.9.10`, the instrument), and running them when asked.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · law | L6: the `tailwind` pack on both registries (twenty-six packs: nineteen npm, eighteen PyPI); providers (decision 14) — npm only, the release's version; a pack's one dependency on its own provider (decision 4); Node at run time for a provider (decision 3's second exception); pack parity compares files, not the manifest's dependency. L11: D-50 (`tailwindcss`, the provider's peer dependency, the consumer's own) and D-51 (the pack's dependency on its provider). CLAUDE.md's and SR-LAWS's L6 line | ✅ |
| 2 · provider | `packaging/npm/provider-tailwind/index.mjs`: asks the project's `tailwindcss` design system for order, properties, variants, important and — only when `tailwind/enforces-canonical-classname` is on — the canonical spelling. `crates/codelaw-engine/src/providers.rs` finds it in `node_modules`, runs it with Node, and caches the table at `.codelaw/cache/providers/tailwind.json` keyed by the provider version, the request and the SHA-256 of every file the provider read. **v4 only:** v3 has no design-system API and gets the `on_missing` note; a v3 path needs a v3 Tailwind ratified to measure it (question below). Times (release build, a loaded machine): 59 files cold 0.4 s / warm 0.08 s; 2,420 files cold 9.5 s / warm 0.44 s, the table alone 4.3 s (SR-ARCHITECTURE decision 28a) | ✅ |
| 2a · missing provider | `[providers] on_missing` (`skip` default, `error`), `[providers.tailwind] on_missing` and `stylesheet`; `skip` adds one note naming the package; `codelaw init` adds the pack and `@codelaw/provider-tailwind` from npm (a PyPI install gets a note) | ✅ |
| 3 · sorting | byte-identical to `prettier-plugin-tailwindcss` on ≥ 98 % of a Tailwind corpus — `scripts/tailwind-parity.py`, corpus shadcn/ui `apps/v4` (tag `shadcn@3.8.5`, commit `ad25490c`, MIT) and Headless UI's React playground (tag `@headlessui/react@v2.2.10`, commit `d13526d0`, MIT) — **sorting identical on 2,420 of 2,420 and 59 of 59 files (100 %)**, of which the plugin re-sorts 84 and 44; the whole `codelaw format` identical on 99.6 % and 100 % (Biome's layout is the rest). SR-FORMATTER decision 9 has the table | ✅ |
| 4 · names | npm first publish of `@codelaw/rules-tailwind` and `@codelaw/provider-tailwind`, PyPI pending publisher for `codelaw-rules-tailwind` | 🔴 handed to [W-142](../../work/open/W-142-framework-pack-names.md) (its table, added 2026-11-05) |
| 5 · records, docs | SR-FORMATTER decision 9 (`[format.tailwind]`), SR-RULE-PACKS decision 19 (`providers`, `[providers]`, the pack), SR-PACKAGES decision 3c (the provider kind), SR-CONFIG §1 and decision 3 (`[providers]`), SR-ARCHITECTURE decisions 28 and 28a (running and caching a provider; `codelaw_core::class_table`), SR-RULE-FORMAT decision 1 (the `tailwind` namespace), the register; GLOSSARY (provider, coined 2026-10-30); CHANGELOG; the pack's README; rule pages | ✅ |

## What was built

- **`codelaw_core::class_table`** — `ClassTable`, `ClassInfo`, and `sort_classes`, the sort written fresh from prettier-plugin-tailwindcss 0.8.1's documented behaviour (unknown first, Tailwind's order, `...` last, known duplicates dropped, whitespace collapsed except where joined by `+` or `${…}`).
- **`codelaw_lang::class_lists`** — where class lists are: `class`/`className` (HTML, JSX), strings and templates inside a `{…}`, the arguments of `clsx`/`cn`/`cva`/`tw` (through member accesses and calls), tagged templates. One finder for the sorter and the rules.
- **Formatter** — `FormatOptions::class_sort`, `[format.tailwind] sort | attributes | functions`; lists sorted before the engine lays out. `codelaw debug sort-classes [--write]` sorts without layout (what the parity script measures).
- **Provider** — `@codelaw/provider-tailwind` (`packaging/npm/provider-tailwind/`), peer dependency `tailwindcss >=4.0.0`; `scripts/package.py` builds it (`PROVIDERS`), gives `@codelaw/rules-tailwind` its one exact dependency and checks both; `publish.yml` uploads it in the binaries' tier.
- **Engine** — `providers.rs` (find, run, cache, `on_missing`), `FileContext::class_table` for native rules, the tables' identity in every file's cache key; `CODELAW_PROVIDER_ROOT` for codelaw's own tests and the rule tester, which reports provider rules `not checked` without one.
- **The pack** — `packs/tailwind/`, both registries, eight `native:` rules in `crates/codelaw-native/src/tailwind.rs`, `tailwind/recommended` (five: `no-conflicting-classes` at `error`, the rest `warn`). `codelaw test packs/tailwind`: 258 checks, 0 failed with `CODELAW_PROVIDER_ROOT=fixtures/tailwind`; without it the three table rules are `not checked`.
- **`fixtures/tailwind`** — `tailwindcss@4.3.3` pinned by `package-lock.json` (D-48); `npm ci` there, then link the provider (its `package.json` says how).

## What changed from the filing

- **`tailwind/enforces-canonical-classname` is opt-in**, not in `recommended`: Tailwind indexes canonical forms on first use in every provider run, 3 s on an idle machine and 19 s on a loaded one (2026-11-05). `important-modifier-suffix` and `enforces-negative-arbitrary-values` cover the common respellings without it.
- **`no-conflicting-classes` is `suggest`**, not the source's `auto` (L4): the losing class does nothing in Tailwind's CSS, but a selector of the project's own could still read it.
- **The stylesheet** is found by the provider when `[providers.tailwind] stylesheet` is unset (the first `.css` file importing "tailwindcss", else Tailwind's `theme.css`).

## Questions for Arpit

1. **Tailwind v3.** The provider answers v4 only (the design-system API). v3 projects get the `on_missing` note. Supporting v3 means its own path (v3's `createContext(...).getClassOrder`) and measuring it on a v3 Tailwind, which would be a new fixture dependency (`tailwindcss@3.x`) to ratify under L11. Build it, or leave v3 out?

## Record affected

SR-LAW-6 · SR-LAW-11 · SR-LAWS · [SR-FORMATTER](../../records/0105_formatter.md) · SR-RULE-PACKS · SR-PACKAGES · SR-CONFIG · SR-ARCHITECTURE · SR-RULE-FORMAT · the register.

## Reference

[W-144](W-144-rule-parity-round-2.md) · [W-141](W-141-formatter-parity.md) · [W-142](../../work/open/W-142-framework-pack-names.md) · prettier-plugin-tailwindcss (MIT)
