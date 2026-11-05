---
id: react/a11y-tabindex-no-positive
title: "No positive tabIndex"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`tabIndex={0}` puts the element in document order, which moves it in the tab sequence; where the order mattered, the markup is reordered instead"
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
fix: "tabIndex={0}"
```

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
