---
id: js/use-json-import-attributes
title: "A JSON import carries `with { type: \"json\" }`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the attribute needs a runtime and a bundler that read import attributes, which older toolchains do not"
stability: preview
ported_from: [biome/correctness/useJsonImportAttributes]
since: 0.5.1
---

An `import … from "x.json"` has an import attribute, `with { type: "json" }`. Behaviour of Biome's `useJsonImportAttributes`.

**Why:** Node.js and browsers refuse to load a JSON module without the attribute: it says the file is data, not code, so a server cannot swap in a script.

```codelaw
match:
  kind: import_statement
  all:
    - has: { field: source, kind: string, regex: '\.json["'']$', pattern: $S }
    - not: { has: { kind: import_attribute } }
message: "Add `with { type: \"json\" }` to this JSON import"
fix:
  - insert-after: { target: $S, text: ' with { type: "json" }' }
```

## Examples

### JavaScript

#### Don't

```js
import data from "./data.json";
```

#### Do

```js
import data from "./data.json" with { type: "json" };
```

### JSX

#### Don't

```jsx
import data from "./data.json";
```

#### Do

```jsx
import data from "./data.json" with { type: "json" };
```

### TypeScript

#### Don't

```ts
import data from "./data.json";
```

#### Do

```ts
import data from "./data.json" with { type: "json" };
```

### TSX

#### Don't

```tsx
import data from "./data.json";
```

#### Do

```tsx
import data from "./data.json" with { type: "json" };
```
