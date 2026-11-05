---
id: next/no-head-import-in-document
title: "No next/head in _document"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the reviewer confirms `Head` in the document is the one `next/document` exports (offered only for a plain `import Head from`)"
files: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-head-import-in-document"]
since: 0.5.0
---

`pages/_document` takes `Head` from `next/document`, not `next/head`.

**Why:** `next/head` is for pages; in the document it does nothing useful and Next.js warns.

```codelaw
match:
  kind: import_statement
  regex: 'from\s*["'']next/head["'']'
  any:
    - all:
      - has: {kind: import_clause, regex: '^Head$', pattern: $CLAUSE}
      - has: {field: source, pattern: $SRC}
    - kind: import_statement
message: "Import `Head` from `next/document` in `_document`"
fix:
  - replace: { target: $CLAUSE, with: "{ Head }" }
  - replace: { target: $SRC, with: '"next/document"' }
```

## How to fix

1. Replace the import with `import { Head } from 'next/document'`.

## Examples

### JavaScript

#### Don't

```js path=pages/_document.js
import Head from "next/head";
```

#### Do

```js path=pages/_document.js
import { Head } from "next/document";
```

### JSX

#### Don't

```jsx path=pages/_document.jsx
import Head from "next/head";
```

#### Do

```jsx path=pages/_document.jsx
import { Head } from "next/document";
```

### TypeScript

#### Don't

```ts path=pages/_document.ts
import Head from "next/head";
```

#### Do

```ts path=pages/_document.ts
import { Head } from "next/document";
```

### TSX

#### Don't

```tsx path=pages/_document.tsx
import Head from "next/head";
```

#### Do

```tsx path=pages/_document.tsx
import { Head } from "next/document";
```
