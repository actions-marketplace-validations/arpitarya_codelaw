---
id: next/no-css-tags
title: "No stylesheet link tags for local CSS"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "importing the CSS moves it into the module graph"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-css-tags"]
since: 0.5.0
---

A local stylesheet is imported, not added with `<link rel="stylesheet">`.

**Why:** An imported stylesheet is bundled, optimised and code-split; a link tag bypasses all of that.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^link$}
  - has: {kind: jsx_attribute, regex: '^rel=["'']stylesheet["'']'}
  - has: {kind: jsx_attribute, regex: '^href=["''](?:[^h/]|/[^/])'}
message: Import the stylesheet instead of linking it
```

## How to fix

1. `import './styles.css'` in the page or layout, and remove the `<link>`.

## Examples

### JavaScript

#### Don't

```js
const l = <link rel="stylesheet" href="/styles.css" />;
```

#### Do

```js
const l = <link rel="stylesheet" href="https://cdn.example.com/lib.css" />;
```

### JSX

#### Don't

```jsx
const l = <link rel="stylesheet" href="/styles.css" />;
```

#### Do

```jsx
const l = <link rel="stylesheet" href="https://cdn.example.com/lib.css" />;
```

### TSX

#### Don't

```tsx
const l = <link rel="stylesheet" href="/styles.css" />;
```

#### Do

```tsx
const l = <link rel="stylesheet" href="https://cdn.example.com/lib.css" />;
```
