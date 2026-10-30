---
id: next/no-head-element
title: "next/head, not head"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "moving the tags into `next/head` changes the imports"
files: ["**/pages/**"]
exclude: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-head-element"]
since: 0.5.0
---

A page uses `<Head>` from `next/head`, not a raw `<head>` element.

**Why:** Next.js manages the document head; a raw `<head>` in a page is duplicated or ignored.

```codelaw
match:
  any:
    - kind: jsx_opening_element
      has: { field: name, regex: '^head$' }
    - kind: jsx_self_closing_element
      has: { field: name, regex: '^head$' }
message: "Use `<Head>` from `next/head` instead of `<head>`"
```

## How to fix

1. `import Head from 'next/head'` and use `<Head>`.

## Examples

### JavaScript

#### Don't

```js path=pages/index.js
const p = <head><title>Home</title></head>;
```

#### Do

```js path=pages/index.js
const p = <Head><title>Home</title></Head>;
```

### JSX

#### Don't

```jsx path=pages/index.jsx
const p = <head><title>Home</title></head>;
```

#### Do

```jsx path=pages/index.jsx
const p = <Head><title>Home</title></Head>;
```

### TSX

#### Don't

```tsx path=pages/index.tsx
const p = <head><title>Home</title></head>;
```

#### Do

```tsx path=pages/index.tsx
const p = <Head><title>Home</title></Head>;
```
