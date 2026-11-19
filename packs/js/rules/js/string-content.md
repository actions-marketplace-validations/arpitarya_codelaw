---
id: js/string-content
title: "Strings say it the project's way"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
options:
  patterns: { type: "object", default: {}, description: "a regular expression to the text that replaces what it matches: a string, or `{ suggest, fix, caseSensitive, message }`" }
ported_from: [eslint-plugin-unicorn/string-content]
since: 0.5.2
---

Text the project does not want in a string literal or a template's text is replaced: `patterns = { "\\.\\.\\." = "…" }` turns `"Loading..."` into `"Loading…"`. A key is a regular expression; its value is the replacement, or an object with `suggest` (the replacement), `fix` (`false` makes the edit a suggestion instead of an automatic fix), `caseSensitive` (`false` ignores case) and `message` (which may use `{{match}}` and `{{suggest}}`; the default is ``Prefer `{{suggest}}` over `{{match}}`.``). The first pattern that matches a string decides, and every match in it is replaced. Template text is read between its substitutions, and `gql`, `html`, `sql`, `styled` and `svg` templates, which hold another language, are not read. Behaviour of eslint-plugin-unicorn 79.0.0's rule for JavaScript, read and reimplemented: a string is read as written, so a match that overlaps an escape sequence (`\n`, `\u2019`) is not reported, and the CSS, TOML and YAML halves are other packs' rules; its `selectors` option, which names ESTree nodes, has no equivalent. A pattern that is not a valid regular expression is reported once, on the file's first string.

With no `patterns` set the rule is silent: it reports nothing, not even that the option is missing, and the engine does not run it on a single node. It is in no preset; a project switches it on by setting the option under `[rules]` (or in a `[scope.<name>]` for some files).

In `codelaw.toml`, under `[rules]`: `"js/string-content" = { severity = "warn", options = { patterns = { "\\.\\.\\." = "…" } } }`.

**Why:** Typography, spelling and house style in user-facing strings are a list a project keeps; the rule applies it.

```codelaw
native: js/string-content
```

## Examples

### JavaScript

#### Don't

```js options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading...";
```

#### Do

```js options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

#### Fixed

```js options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

### JSX

#### Don't

```jsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading...";
```

#### Do

```jsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

#### Fixed

```jsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

### TypeScript

#### Don't

```ts options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading...";
```

#### Do

```ts options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

#### Fixed

```ts options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

### TSX

#### Don't

```tsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading...";
```

#### Do

```tsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```

#### Fixed

```tsx options={"patterns":{"\\\\.\\\\.\\\\.":"…"}}
const loading = "Loading…";
```
