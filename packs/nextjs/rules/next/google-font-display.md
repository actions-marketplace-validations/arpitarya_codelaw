---
id: next/google-font-display
title: "Google Fonts load with display=swap or optional"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "`swap` and `optional` trade layout shift against fallback text differently; the author picks"
fix_pending: "suggest-choices; needs computed-edit (W-162)"
stability: preview
ported_from: ["@next/eslint-plugin-next/google-font-display"]
since: 0.5.0
---

A Google Fonts stylesheet link sets `display=swap` or `display=optional`.

**Why:** Without it, text stays invisible while the font loads (or swaps late with `block`/`auto`), which hurts first paint.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^link$}
  - has: {kind: jsx_attribute, regex: ^href=.*fonts\.googleapis\.com/css}
  - not:
      has: {kind: jsx_attribute, regex: '^href=.*[?&]display=(swap|optional)\b'}
message: Add `&display=swap` (or `optional`) to the Google Fonts URL, or use `next/font`
```

## How to fix

1. Append `&display=swap` (or `&display=optional`) to the URL, or move the font to `next/font/google`.

## Examples

### JavaScript

#### Don't

```js
const l = <link href="https://fonts.googleapis.com/css2?family=Inter" rel="stylesheet" />;
```

#### Do

```js
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```

### JSX

#### Don't

```jsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter" rel="stylesheet" />;
```

#### Do

```jsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```

### TSX

#### Don't

```tsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter" rel="stylesheet" />;
```

#### Do

```tsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet" />;
```
