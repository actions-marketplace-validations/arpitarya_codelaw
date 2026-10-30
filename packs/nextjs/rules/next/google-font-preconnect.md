---
id: next/google-font-preconnect
title: "fonts.gstatic.com is preconnected"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the link's purpose is the author's to confirm"
stability: preview
ported_from: ["@next/eslint-plugin-next/google-font-preconnect"]
since: 0.5.0
---

A `<link>` to `fonts.gstatic.com` has `rel="preconnect"`.

**Why:** Preconnecting opens the connection to the font host early, so the font file starts downloading sooner.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^link$}
  - has: {kind: jsx_attribute, regex: ^href=.*fonts\.gstatic\.com}
  - not:
      has: {kind: jsx_attribute, regex: '^rel=["'']preconnect["'']'}
message: Add `rel="preconnect"` to the link to fonts.gstatic.com
```

## How to fix

1. Add `rel="preconnect"` (and `crossOrigin=""`) to the link.

## Examples

### JavaScript

#### Don't

```js
const l = <link href="https://fonts.gstatic.com" />;
```

#### Do

```js
const l = <link href="https://fonts.gstatic.com" rel="preconnect" crossOrigin="" />;
```

### JSX

#### Don't

```jsx
const l = <link href="https://fonts.gstatic.com" />;
```

#### Do

```jsx
const l = <link href="https://fonts.gstatic.com" rel="preconnect" crossOrigin="" />;
```

### TSX

#### Don't

```tsx
const l = <link href="https://fonts.gstatic.com" />;
```

#### Do

```tsx
const l = <link href="https://fonts.gstatic.com" rel="preconnect" crossOrigin="" />;
```
