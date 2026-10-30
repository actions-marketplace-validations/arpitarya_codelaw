---
type: Handoff
name: W-157
description: "Arpit, 2026-10-30: autocomplete and live checks in codelaw.toml — keys, real rule ids from the installed packs, presets, severities, options, the down-only fix key — from codelaw's own language server."
item: W-157
filed: 2026-10-30
filed_by: "Arpit (Claude app, Opus 5.5)"
updated: 2026-10-30
updated_by: "Arpit (Claude app, Opus 5.5)"
ball: agent
---

# W-157 — codelaw completes and checks its own `codelaw.toml` as you type

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | The extension registers `codelaw.schema.json` for `codelaw.toml`, and `init` writes the `#:schema` line | keys and values complete today, through Even Better TOML | ⏳ |
| 2 | The language server opens `codelaw.toml` itself: completions from the installed packs | real rule ids, presets, options; nothing a fixed schema can't know | ⏳ |
| 3 | The config loader's errors appear as you type, with "did you mean" quick fixes | a typo is red before the run, not at exit 2 | ⏳ |

**Arpit, 2026-10-30 (voice):** *"Is there a way whosoever is using it can get suggestions that, okay, this is the property you can add … this is the value that you should use … like autocomplete."* On the language server reading its own config: *"I would love that. Code law could look into even its own code law.toml file."*

**Model:** Sonnet: each completion source and error has a test, and the LSP already serves completions and diagnostics for code.

## Goal

Writing `codelaw.toml` is as guided as writing code: every key, rule id, preset, severity, option and `fix` value is offered from what is actually installed, and every config error shows at the line while typing.

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · schema now | the extension contributes `codelaw.schema.json` for `codelaw.toml` (Even Better TOML's `tomlValidation` contribution; check it before relying on it); `codelaw init`'s template writes `#:schema ./node_modules/codelaw/codelaw.schema.json`, or the PyPI path, by registry (L10) | ⏳ |
| 2 · the server opens it | the LSP handles `codelaw.toml`, `[tool.codelaw]` in `pyproject.toml` and `"codelaw"` in `package.json` (SR-CONFIG's three homes) | ⏳ |
| 3 · completions | section and key names from the schema; rule ids in `[rules]` and scope `rules` from the loaded packs and `.codelaw/rules`; `codelaw:<ns>/<preset>` in `extends` from the installed packs; severities; a rule's `options` keys and enum values from its frontmatter; `fix` offering only the rule's rung and below (SR-LAW-4 decision 16); scope names in `[scope.<name>]` from the presets' scopes | ⏳ |
| 4 · hover | a rule id shows its title, fix rung, pack and version, and the link to its page; a key shows its schema description | ⏳ |
| 5 · live errors | every config load error (SR-CONFIG: unknown key, wrong type, unknown rule, missing pack, root-only key in a nested config, a raised `fix`) is a diagnostic at its line, with a "did you mean" quick fix where the loader has one | ⏳ |
| 6 · no drift | completions and errors come from the same loader and pack search the run uses (SR-PACK-LOOKUP), never a second list | ⏳ |

## Blockers

None. DoD 3's `fix` values wait on W-156 A6.

## Record affected

SR-EDITOR (the server handles the config files: a new decision); SR-CONFIG (the `#:schema` line in `init`'s template); SR-LAW-10 (the template change).

## Hazards

- **TOML positions.** Completion needs the cursor's table path, even in a half-typed file that doesn't parse. Use a tolerant read, not the strict loader.
- **Three homes.** In `pyproject.toml` and `package.json`, offer codelaw's keys only inside its own table or object.
- **Missing packs.** If a pack isn't installed, its rule ids can't be completed. Say so in the completion list's detail; don't guess ids.

## Reference

- `packaging/vscode/package.json`: no schema contribution today.
- `crates/codelaw-lsp/src/`: the server, which handles code files only today.
- `templates/init/codelaw.toml`: no `#:schema` line.
- [SR-CONFIG](../../records/0110_config.md) §1: its own example already starts with `#:schema`.
