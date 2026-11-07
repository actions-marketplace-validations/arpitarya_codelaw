---
id: next/google-font-display
title: "Google Fonts load with display=swap or optional"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`swap` and `optional` trade layout shift against fallback text differently, so both are offered and the author picks; a `display` already in the URL is replaced, and a new parameter is joined with `&amp;`, which JSX reads as `&`"
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
      has: {kind: jsx_attribute, regex: '^href=.*[?&;]display=(swap|optional)\b'}
  - any:
    - has: {kind: jsx_attribute, regex: '^href=.*[?&;]display=', pattern: $SET}
    - has: {kind: jsx_attribute, regex: '^href=.*fonts\.googleapis\.com/css', not: {regex: '[?&;]display='}, pattern: $ADD}
message: Add `&display=swap` (or `optional`) to the Google Fonts URL, or use `next/font`
fix:
  choices:
    - label: swap
      fix:
        - edit-text: { target: $SET, pattern: '([?&;]display=)[\w-]*', replace: '${1}swap' }
        - edit-text: { target: $ADD, pattern: '(fonts\.googleapis\.com/css2?\?[^"''\s]*)', replace: '${1}&amp;display=swap' }
        - edit-text: { target: $ADD, pattern: '(fonts\.googleapis\.com/css2?)(["''])', replace: '${1}?display=swap${2}' }
    - label: optional
      fix:
        - edit-text: { target: $SET, pattern: '([?&;]display=)[\w-]*', replace: '${1}optional' }
        - edit-text: { target: $ADD, pattern: '(fonts\.googleapis\.com/css2?\?[^"''\s]*)', replace: '${1}&amp;display=optional' }
        - edit-text: { target: $ADD, pattern: '(fonts\.googleapis\.com/css2?)(["''])', replace: '${1}?display=optional${2}' }
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

#### Fixed (swap)

```js
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=swap" rel="stylesheet" />;
```

#### Fixed (optional)

```js
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=optional" rel="stylesheet" />;
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

#### Fixed (swap)

```jsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=swap" rel="stylesheet" />;
```

#### Fixed (optional)

```jsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=optional" rel="stylesheet" />;
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

#### Fixed (swap)

```tsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=swap" rel="stylesheet" />;
```

#### Fixed (optional)

```tsx
const l = <link href="https://fonts.googleapis.com/css2?family=Inter&amp;display=optional" rel="stylesheet" />;
```
