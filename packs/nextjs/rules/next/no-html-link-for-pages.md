---
id: next/no-html-link-for-pages
title: "Link, not a, for internal pages"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`<Link>` navigates on the client and prefetches, where `<a>` reloads the page; `Link` is imported from `next/link` where the file does not, and a file that binds `Link` to something else is fixed by hand"
files: ["**/pages/**", "**/app/**", "**/src/**", "**/components/**"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-html-link-for-pages"]
since: 0.5.0
---

A link to a page of the app uses `<Link>` from `next/link`, not `<a href="/…">`.

**Why:** A plain anchor reloads the whole app; `<Link>` navigates on the client and prefetches.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^a$, pattern: $OPEN}
  - any:
    - {kind: jsx_self_closing_element}
    - inside: {kind: jsx_element, has: {kind: jsx_closing_element, has: {field: name, pattern: $CLOSE}}}
  - has: {kind: jsx_attribute, regex: '^href=["'']/(?:[^/]|$)'}
  - not:
      has: {kind: jsx_attribute, regex: ^(target|download)\b}
message: Use `<Link>` from `next/link` for links inside the app
fix:
  - replace: { target: $OPEN, with: Link }
  - replace: { target: $CLOSE, with: Link }
  - ensure-import: { module: next/link, name: default, as: Link }
```

## How to fix

1. `import Link from 'next/link'` and replace `<a>` with `<Link>`.

## Examples

### JavaScript

#### Don't

```js path=pages/index.js
const a = <a href="/about">About</a>;
```

#### Do

```js path=pages/index.js
import Link from "next/link";

const a = <Link href="/about">About</Link>;
```

### JSX

#### Don't

```jsx path=pages/index.jsx
const a = <a href="/about">About</a>;
```

#### Do

```jsx path=pages/index.jsx
import Link from "next/link";

const a = <Link href="/about">About</Link>;
```

### TSX

#### Don't

```tsx path=pages/index.tsx
const a = <a href="/about">About</a>;
```

#### Do

```tsx path=pages/index.tsx
import Link from "next/link";

const a = <Link href="/about">About</Link>;
```

## Differences from the source

The source reads the `pages` folder to know which paths are pages; this rule treats every root-relative `href` as one.
