---
id: next/no-head-element
title: "next/head, not head"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`<Head>` moves its children into the document's head when the page renders, which a `<head>` in a page does not; `Head` is imported from `next/head` where the file does not, and a file that binds `Head` to something else is fixed by hand"
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
      has: { field: name, regex: '^head$', pattern: $OPEN }
      inside: { kind: jsx_element, has: { kind: jsx_closing_element, has: { field: name, pattern: $CLOSE } } }
    - kind: jsx_self_closing_element
      has: { field: name, regex: '^head$', pattern: $OPEN }
message: "Use `<Head>` from `next/head` instead of `<head>`"
fix:
  - replace: { target: $OPEN, with: Head }
  - replace: { target: $CLOSE, with: Head }
  - ensure-import: { module: next/head, name: default, as: Head }
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
import Head from "next/head";
const p = <Head><title>Home</title></Head>;
```

### JSX

#### Don't

```jsx path=pages/index.jsx
const p = <head><title>Home</title></head>;
```

#### Do

```jsx path=pages/index.jsx
import Head from "next/head";
const p = <Head><title>Home</title></Head>;
```

### TSX

#### Don't

```tsx path=pages/index.tsx
const p = <head><title>Home</title></head>;
```

#### Do

```tsx path=pages/index.tsx
import Head from "next/head";
const p = <Head><title>Home</title></Head>;
```
