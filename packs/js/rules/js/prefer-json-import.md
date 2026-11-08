---
id: js/prefer-json-import
title: "Import a JSON file rather than reading and parsing it at the top level"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an import resolves the path from the module, where `readFileSync` resolves it from the working directory, and loads it once at start-up; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-json-import]
since: 0.5.0
---

A module-level `const data = JSON.parse(readFileSync('./data.json', 'utf8'))` (or `fs.readFileSync`, with `'utf8'`, `'utf-8'` or no encoding) is `import data from './data.json' with { type: 'json' };`. The path is a `.json` string literal; a computed path is left.

**Why:** The import says the module depends on the data, and bundlers and type checkers can see it.

```codelaw
match:
  kind: lexical_declaration
  inside: { kind: program }
  any:
    - pattern: "const $D = JSON.parse($R($P, $E));"
    - pattern: "const $D = JSON.parse($R($P));"
constraints:
  R: { regex: '^(fs\.)?readFileSync$' }
  P: { kind: string, regex: '\.json["'']$' }
  D: { kind: identifier }
message: "Import `$P` as JSON"
fix: "import $D from $P with { type: \"json\" };"
```

## Examples

### JavaScript

#### Don't

```js
const settings = JSON.parse(fs.readFileSync("./settings.json", "utf8"));
```

#### Do

```js
import settings from "./settings.json" with { type: "json" };
```


### JSX

#### Don't

```jsx
const settings = JSON.parse(fs.readFileSync("./settings.json", "utf8"));
```

#### Do

```jsx
import settings from "./settings.json" with { type: "json" };
```


### TypeScript

#### Don't

```ts
const settings = JSON.parse(fs.readFileSync("./settings.json", "utf8"));
```

#### Do

```ts
import settings from "./settings.json" with { type: "json" };
```


### TSX

#### Don't

```tsx
const settings = JSON.parse(fs.readFileSync("./settings.json", "utf8"));
```

#### Do

```tsx
import settings from "./settings.json" with { type: "json" };
```
