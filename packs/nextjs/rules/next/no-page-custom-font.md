---
id: next/no-page-custom-font
title: "Custom fonts in the document, not a page"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "moving the font changes the app's font setup"
files: ["**/pages/**"]
exclude: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-page-custom-font"]
since: 0.5.0
---

A Google Fonts stylesheet is not linked from a single page.

**Why:** A font linked in one page is loaded only there and re-fetched on navigation; `next/font` or the document loads it once.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^link$}
  - has: {kind: jsx_attribute, regex: ^href=.*fonts\.googleapis\.com}
message: Load custom fonts with `next/font` or in `_document`, not in a page
```

## How to fix

1. Use `next/font/google`, or move the link to `pages/_document`.

## Examples

### JavaScript

#### Don't

```js path=pages/index.js
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```

#### Do

```js path=pages/index.js
const inter = Inter({ subsets: ["latin"] });
```

### JSX

#### Don't

```jsx path=pages/index.jsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```

#### Do

```jsx path=pages/index.jsx
const inter = Inter({ subsets: ["latin"] });
```

### TSX

#### Don't

```tsx path=pages/index.tsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```

#### Do

```tsx path=pages/index.tsx
const inter = Inter({ subsets: ["latin"] });
```
