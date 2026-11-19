---
id: data/key-name-casing
title: "Keys in a data file use one casing style"
languages: [json, yaml, toml]
fix: suggest
fix_reason: "renaming a key changes what every reader of the file sees; the diff is deterministic, the reviewer decides"
stability: preview
options:
  case: { type: "string", default: "", description: "the casing every key uses: `camelCase`, `PascalCase`, `snake_case`, `kebab-case` or `SCREAMING_SNAKE_CASE`" }
  ignore: { type: "string[]", default: [], description: "keys that may be written any way: a name as written, or `/pattern/` (`i` is the one flag read)" }
ported_from: [eslint-plugin-unicorn/key-name-casing]
since: 0.5.2
---

Every key of a JSON object, a YAML mapping or a TOML table is written in the style `case` names: `case = "camelCase"` reports `max_retries` and `MaxRetries`, and suggests `maxRetries`. A key is split into words at `_`, at `-` and at a step from lower to upper case (`userID` is `user` and `id`), then written back in the style, so `userID` is not `camelCase` here, `userId` is. In TOML a table header (`[tool.my_tool]`) and each part of a dotted key is a key. A key with a character outside ASCII letters, digits, `_` and `-` (`$schema`, `@type`, `.github`, a key with a space) is not made of words and is not checked, nor is a YAML key with an anchor, a tag or an escape. Behaviour of eslint-plugin-unicorn 79.0.0's `key-name-casing`, read and reimplemented.

With no `case` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. A `case` that is not one of the five is reported once, on the first key. It is in no preset; a project switches it on under `[rules]`, usually in a `[scope.<name>]` for the files whose keys are its own, since a file that follows another tool's schema keeps that tool's casing (`package.json`'s `devDependencies`, a workflow's `runs-on`).

In `codelaw.toml`, under `[rules]`: `"data/key-name-casing" = { severity = "warn", options = { case = "camelCase", ignore = ["/^[A-Z_]+$/"] } }`.

**Why:** A config file read by hand and by code is easier to read, and to search, when its keys follow one style.

```codelaw
native: data/key-name-casing
```

## How to fix

1. Rename the key to the suggested spelling.
2. Rename it wherever code or another file reads it; a rename the reader does not know about breaks it.
3. If the key belongs to another tool's schema, add it to `ignore` or scope the rule away from the file.

## Examples

### JSON

#### Don't

```json options={"case":"camelCase"}
{ "max_retries": 3 }
```

#### Do

```json options={"case":"camelCase"}
{ "maxRetries": 3 }
```

### YAML

#### Don't

```yaml options={"case":"snake_case"}
maxRetries: 3
```

#### Do

```yaml options={"case":"snake_case"}
max_retries: 3
```

### TOML

#### Don't

```toml options={"case":"kebab-case"}
[build_options]
max_retries = 3
```

#### Do

```toml options={"case":"kebab-case"}
[build-options]
max-retries = 3
```
