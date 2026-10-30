---
id: next/no-title-in-document-head
title: "No title in _document's Head"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "each page's title is the author's"
files: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-title-in-document-head"]
since: 0.5.0
---

`<title>` is not placed in `pages/_document`'s `<Head>`.

**Why:** A title in the document applies to every page and cannot change per page; `next/head` in each page sets it.

```codelaw
match:
  any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
  has: { field: name, regex: '^title$' }
  inside:
    stopBy: end
    kind: jsx_element
    has: { field: open_tag, has: { field: name, regex: '^Head$' } }
message: "Set `<title>` in each page with `next/head`, not in `_document`"
```

## How to fix

1. Remove the `<title>` and set it in each page with `next/head`.

## Examples

### JavaScript

#### Don't

```js path=pages/_document.js
const d = <Head><title>Site</title></Head>;
```

#### Do

```js path=pages/_document.js
const d = <Head><meta charSet="utf-8" /></Head>;
```

### JSX

#### Don't

```jsx path=pages/_document.jsx
const d = <Head><title>Site</title></Head>;
```

#### Do

```jsx path=pages/_document.jsx
const d = <Head><meta charSet="utf-8" /></Head>;
```

### TSX

#### Don't

```tsx path=pages/_document.tsx
const d = <Head><title>Site</title></Head>;
```

#### Do

```tsx path=pages/_document.tsx
const d = <Head><meta charSet="utf-8" /></Head>;
```
