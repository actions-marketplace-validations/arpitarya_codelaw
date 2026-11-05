---
id: js/no-barrel-files
title: "No barrel file"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "every importer of the barrel must import from the real module"
stability: preview
ported_from: [eslint-plugin-unicorn/no-barrel-files]
since: 0.5.0
---

A file does not consist only of re-exports (`export … from`).

**Why:** Barrel files make every importer load every module behind them and slow down tools.

```codelaw

match:
  kind: program
  has: { kind: export_statement, has: { field: source, regex: "." } }
  not: { has: { not: { any: [{ kind: comment }, { kind: export_statement, has: { field: source, regex: "." } }] } } }
message: "This file only re-exports; import from the modules themselves"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
export * from "./a.js";
export { b } from "./b.js";
```

#### Do

```js
export const c = 1;
```

### JSX

#### Don't

```jsx
export * from "./a.js";
export { b } from "./b.js";
```

#### Do

```jsx
export const c = 1;
```

### TypeScript

#### Don't

```ts
export * from "./a.js";
export { b } from "./b.js";
```

#### Do

```ts
export const c = 1;
```

### TSX

#### Don't

```tsx
export * from "./a.js";
export { b } from "./b.js";
```

#### Do

```tsx
export const c = 1;
```
