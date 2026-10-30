---
id: next/no-document-import-in-page
title: "next/document only in _document"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement — `next/head` or the layout — depends on what was imported"
exclude: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-document-import-in-page"]
since: 0.5.0
---

`next/document` is imported only in `pages/_document`.

**Why:** Its components only work while the document renders; imported anywhere else they break the page.

```codelaw
match:
  kind: import_statement
  regex: 'from\s*["'']next/document["'']'
message: "Import `next/document` only in `pages/_document`"
```

## How to fix

1. Use `next/head` in a page, or move the code into `pages/_document`.

## Examples

### JavaScript

#### Don't

```js
import { Html } from "next/document";
```

#### Do

```js
import Head from "next/head";
```

### JSX

#### Don't

```jsx
import { Html } from "next/document";
```

#### Do

```jsx
import Head from "next/head";
```

### TypeScript

#### Don't

```ts
import { Html } from "next/document";
```

#### Do

```ts
import Head from "next/head";
```

### TSX

#### Don't

```tsx
import { Html } from "next/document";
```

#### Do

```tsx
import Head from "next/head";
```
