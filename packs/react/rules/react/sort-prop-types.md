---
id: react/sort-prop-types
title: "Prop types are in alphabetical order"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "reordering the entries reorders the evaluation of their values"
stability: preview
options:
  ignore_case: { type: "boolean", default: false, description: "compare names without case" }
ported_from: [eslint-plugin-react/sort-prop-types]
since: 0.5.0
---

The keys of `propTypes` are in alphabetical order.

**Why:** A sorted list is easy to scan.

```codelaw
native: react/sort-prop-types
```

## How to fix

1. Sort the keys.

## Examples

### JavaScript

#### Don't

```js
Button.propTypes = { size: n, color: s };
```

#### Do

```js
Button.propTypes = { color: s, size: n };
```

### JSX

#### Don't

```jsx
Button.propTypes = { size: n, color: s };
```

#### Do

```jsx
Button.propTypes = { color: s, size: n };
```

### TSX

#### Don't

```tsx
Button.propTypes = { size: n, color: s };
```

#### Do

```tsx
Button.propTypes = { color: s, size: n };
```
