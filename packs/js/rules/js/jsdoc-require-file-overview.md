---
id: js/jsdoc-require-file-overview
title: "Each file starts with an @file block"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the file overview is the author's to write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-file-overview]
since: 0.5.0
---

Every file has a top-level JSDoc block with `@file` (or its aliases `@fileoverview`, `@overview`). Only the missing case is reported; upstream also reports a duplicate or a misplaced overview.

**Why:** A file overview tells a reader what the module is for before they read its code, and documentation tools group files by it.

```codelaw
match:
  kind: program
  has: {regex: '[\s\S]'}
  not:
    has:
      kind: comment
      regex: '^/\*\*[^*/][\s\S]*@(file|fileoverview|overview)\b'
message: "Add a `/** @file … */` block at the top saying what this file is for"
```

## How to fix

1. Add a JSDoc block at the top of the file: `/** @file What this module does. */`.

## Examples

### JavaScript

#### Don't

```js
export const answer = 42;
```

#### Do

```js
/** @file The answer to everything. */
export const answer = 42;
```

### JSX

#### Don't

```jsx
export const answer = 42;
```

#### Do

```jsx
/** @file The answer to everything. */
export const answer = 42;
```

### TypeScript

#### Don't

```ts
export const answer = 42;
```

#### Do

```ts
/** @file The answer to everything. */
export const answer = 42;
```

### TSX

#### Don't

```tsx
export const answer = 42;
```

#### Do

```tsx
/** @file The answer to everything. */
export const answer = 42;
```
