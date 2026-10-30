---
id: react/boolean-prop-naming
title: "Boolean props are named is… or has…"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the new prop name is the author's, and every caller follows it"
stability: preview
ported_from: [eslint-plugin-react/boolean-prop-naming]
since: 0.5.0
---

A prop declared `PropTypes.bool` has a name starting `is` or `has` (`isOpen`, `hasError`).

**Why:** The prefix tells the reader the prop is a flag without looking up its type.

```codelaw
match:
  kind: pair
  all:
    - has: { field: key, not: { regex: '^["'']?(is|has)[A-Z]' }, pattern: $K }
    - has: { field: value, regex: '^PropTypes\.bool(\.isRequired)?$' }
  inside: { kind: object, inside: { kind: assignment_expression, has: { field: left, regex: '\.propTypes$' } } }
message: "Name the boolean prop `$K` `is…` or `has…`"
```

## How to fix

1. Rename the prop (`open` → `isOpen`) in `propTypes`, in the component and at every call site.

## Examples

### JavaScript

#### Don't

```js
Modal.propTypes = { open: PropTypes.bool };
```

#### Do

```js
Modal.propTypes = { isOpen: PropTypes.bool };
```

### JSX

#### Don't

```jsx
Modal.propTypes = { open: PropTypes.bool };
```

#### Do

```jsx
Modal.propTypes = { isOpen: PropTypes.bool };
```

### TSX

#### Don't

```tsx
Modal.propTypes = { open: PropTypes.bool };
```

#### Do

```tsx
Modal.propTypes = { isOpen: PropTypes.bool };
```
