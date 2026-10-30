---
id: react/forbid-foreign-prop-types
title: "No reading another component's propTypes"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the shape is exported from where it's defined and imported here"
stability: preview
ported_from: [eslint-plugin-react/forbid-foreign-prop-types]
since: 0.5.0
---

Code does not read another component's `propTypes` (`Other.propTypes.size`); assigning a component's own `propTypes` is fine.

**Why:** Production builds strip `propTypes`, so code that reads them breaks only in production.

```codelaw
match:
  kind: member_expression
  regex: '\.propTypes$'
  not: { inside: { kind: assignment_expression, field: left } }
message: "Don't read `propTypes` from another component; export the shape instead"
```

## How to fix

1. Export the prop shape from the component's module and import it where it's needed.

## Examples

### JavaScript

#### Don't

```js
Card.propTypes = { size: Button.propTypes.size };
```

#### Do

```js
Card.propTypes = { size: sizeShape };
```

### JSX

#### Don't

```jsx
Card.propTypes = { size: Button.propTypes.size };
```

#### Do

```jsx
Card.propTypes = { size: sizeShape };
```

### TSX

#### Don't

```tsx
Card.propTypes = { size: Button.propTypes.size };
```

#### Do

```tsx
Card.propTypes = { size: sizeShape };
```
