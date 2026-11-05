---
id: react/no-unused-prop-types
title: "No prop type a component never reads"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing a declared prop changes what callers may pass; the edit is for an agent or a person to accept"
stability: preview
ported_from: [eslint-plugin-react/no-unused-prop-types]
since: 0.5.0
---

A key of a function component's `propTypes` is read by the component, through its destructured parameter or `props.x`. A component that passes `props` on whole is not judged.

**Why:** An unread prop is dead API that callers keep passing.

```codelaw
native: react/no-unused-prop-types
```

## How to fix

1. Remove the prop type, or use the prop.

## Examples

### JavaScript

#### Don't

```js
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s, footer: s };
```

#### Do

```js
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s };
```

### JSX

#### Don't

```jsx
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s, footer: s };
```

#### Do

```jsx
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s };
```

### TSX

#### Don't

```tsx
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s, footer: s };
```

#### Do

```tsx
function Card({ title }) {
  return title;
}
Card.propTypes = { title: s };
```
