---
id: react/a11y-tabindex-no-positive
title: "No positive tabIndex"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the intended order usually means reordering the markup"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/tabindex-no-positive]
since: 0.5.0
---

`tabIndex` is `0` or `-1`, never a positive number.

**Why:** A positive `tabIndex` jumps the element ahead of the page's order, which no one expects and is hard to maintain.

```codelaw
match:
  kind: jsx_attribute
  regex: '^tabIndex=(["'']|\{\s*)\+?0*[1-9]'
message: "Use `tabIndex` 0 or -1; a positive value breaks the page's tab order"
```

## How to fix

1. Use `tabIndex={0}` and put the element where it belongs in the markup.

## Examples

### JavaScript

#### Don't

```js
const s = <span tabIndex={2}>x</span>;
```

#### Do

```js
const s = <span tabIndex={0}>x</span>;
```

### JSX

#### Don't

```jsx
const s = <span tabIndex={2}>x</span>;
```

#### Do

```jsx
const s = <span tabIndex={0}>x</span>;
```

### TSX

#### Don't

```tsx
const s = <span tabIndex={2}>x</span>;
```

#### Do

```tsx
const s = <span tabIndex={0}>x</span>;
```
