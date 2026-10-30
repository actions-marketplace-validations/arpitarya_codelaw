---
type: Handoff
name: W-155
description: "Arpit, 2026-10-30: fall back to the project's .venv when $VIRTUAL_ENV lacks a pack, and make the pack lookup folders settable in codelaw.toml ([packs])."
item: W-155
filed: 2026-10-30
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-10-30
updated_by: "Claude Code (Opus 5.5)"
ball: agent
---

# W-155 — Find packs in the project's `.venv` too, and let `codelaw.toml` set where packs are found

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Write the lookup down as its own record, SR-PACK-LOOKUP | the spec, ruled before building | ✅ 2026-10-30 |
| 2 | Make the engine search the launcher's paths, then `$VIRTUAL_ENV`, then `./.venv`, instead of only the first one set | the editor bug is gone with no settings | ⏳ |
| 3 | Add the `[packs]` section: `order`, `node_modules`, `venv` | a project can pin where its packs come from | ⏳ parsed and validated (2026-10-30); the lookup does not read it yet |
| 4 | Show every searched folder in `config --print`, `rules --why` and the missing-pack error | a wrong lookup can be seen, not guessed | ⏳ |
| 5 | Drop the playground's `codelaw.sh` workaround after the release | proof in a real editor | ⏳ |

**Arpit, 2026-10-30:** *"Let's make that fix … create an SR for how Node and .venv packages and paths are picked … I want both the paths configurable. Default functionality could stay, but the TOML file should have a path."* Ruled the same day: a set key **replaces** its default, and `order` is kept.

**Model:** Sonnet: the record fixes every decision and each DoD row has a test that shows whether it is met.

## Goal

Build [SR-PACK-LOOKUP](../../records/0118_pack-lookup.md) decisions 1–10.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · the fallback | `PackSearch` joins `CODELAW_SITE_PACKAGES` → `$VIRTUAL_ENV` → `./.venv`, drops duplicates and searches in order (decision 4). A unit test sets `VIRTUAL_ENV` to a venv without the pack, puts the pack in `./.venv`, and the pack is found | ⏳ |
| 2 · `[packs]` | parsed and validated in `codelaw-config`, added to `codelaw.schema.json`, root-only; one test per load error in decision 7 | ✅ 2026-10-30: `crates/codelaw-config/src/packs.rs`; main's schema test green again |
| 3 · replace, not prepend | with `venv` set, a `VIRTUAL_ENV` that has the pack is ignored; with `node_modules` set, there is no walk-up; `order = ["pypi"]` never reads `node_modules` (tests) | ⏳ |
| 4 · traceable | `config --print` lists places and their sources; `rules --why` and the JSON name the folder; the missing-pack error lists every folder searched (decision 8) | ⏳ |
| 5 · one lookup | `migrate.rs` and `codelaw-agents/src/package.rs` go through the same function; the veto's grep is clean (decision 9) | ⏳ |
| 6 · ownership | the lookup moves to `crates/codelaw-rules/src/search.rs`. The ownership table gets that row (owner SR-PACK-LOOKUP) and the record's `owns:` gets that path. The `pack.rs::PackSearch,site_packages_of` describes row is removed. The config file that parses `[packs]` gets a describes row | ⏳ |
| 7 · docs | `USAGE.md`'s config reference and `CHANGELOG.md` (SR-WORK-DOCS); the register's `built` → `yes` | ⏳ |
| 8 · playground | on the release that carries this, start VS Code from the codelaw repo's shell with `codelaw.path` and `.vscode/codelaw.sh` removed from codelaw-playground: `rules-python` loads | ⏳ |

## Blockers

None.

## Record affected

SR-PACK-LOOKUP (all), SR-RULE-PACKS decision 4 (now links here), and
SR-CONFIG (`[packs]` in §1's sections, the root-only keys and the validated
sections).

## Hazards

- **Order of precedence.** `CODELAW_SITE_PACKAGES` stays first, so a pip
  install of codelaw keeps loading its own environment's packs. The change only
  adds folders after it.
- **Windows.** Split the path list with `std::env::split_paths`. A venv there is
  `Lib\site-packages`.
- **Folders that don't exist.** A set folder that doesn't exist is a load
  error, but a default one that doesn't exist is skipped silently. Don't mix
  these up.
- **Lockmaster.** `crates/codelaw-rules` is shared with W-139's pack work.
  Claim only the files you change.

## Reference

- [SR-PACK-LOOKUP](../../records/0118_pack-lookup.md)
- `crates/codelaw-rules/src/pack.rs` `PackSearch::new`
- `~/my_programs/codelaw-playground/.vscode/codelaw.sh`: the workaround this
  removes.
