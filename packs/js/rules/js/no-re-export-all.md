---
id: js/no-re-export-all
title: "No `export * from`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the names to export are the author's to list"
fix_exception: restructure
stability: preview
ported_from: [biome/performance/noReExportAll, eslint-plugin-barrel-files/avoid-re-export-all]
since: 0.5.1
---

`export * from "m"` is not written; the names are listed (`export { a, b } from "m"`). `export * as ns from "m"` is a different statement and is left alone. Behaviour of Biome's `noReExportAll`.

**Why:** A wildcard re-export makes bundlers and type checkers load everything the module has, and hides what the file really exports.

```codelaw
match:
  kind: export_statement
  regex: '^export\s+(type\s+)?\*\s*from\b'
message: "List the names to re-export instead of `export *`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
export * from "./helpers";
```

#### Do

```js
export { slugify, titleCase } from "./helpers";
```

### JSX

#### Don't

```jsx
export * from "./helpers";
```

#### Do

```jsx
export { slugify, titleCase } from "./helpers";
```

### TypeScript

#### Don't

```ts
export * from "./helpers";
```

#### Do

```ts
export { slugify, titleCase } from "./helpers";
```

### TSX

#### Don't

```tsx
export * from "./helpers";
```

#### Do

```tsx
export { slugify, titleCase } from "./helpers";
```
