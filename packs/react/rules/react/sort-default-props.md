---
id: react/sort-default-props
title: "Default props are in alphabetical order"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "reordering the entries reorders the evaluation of their values"
stability: preview
options:
  ignore_case: { type: "boolean", default: false, description: "compare names without case" }
ported_from: [eslint-plugin-react/sort-default-props]
since: 0.5.0
---

The keys of `defaultProps` are in alphabetical order.

**Why:** A sorted list is easy to scan.

```codelaw
native: react/sort-default-props
```

## How to fix

1. Sort the keys.

## Examples

### JavaScript

#### Don't

```js
Button.defaultProps = { size: 1, color: "red" };
```

#### Do

```js
Button.defaultProps = { color: "red", size: 1 };
```

### JSX

#### Don't

```jsx
Button.defaultProps = { size: 1, color: "red" };
```

#### Do

```jsx
Button.defaultProps = { color: "red", size: 1 };
```

### TSX

#### Don't

```tsx
Button.defaultProps = { size: 1, color: "red" };
```

#### Do

```tsx
Button.defaultProps = { color: "red", size: 1 };
```
