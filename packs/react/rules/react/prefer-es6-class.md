---
id: react/prefer-es6-class
title: "Class components, not createReactClass"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the component is rewritten as a class or function, with its mixins and autobinding replaced"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/prefer-es6-class]
since: 0.5.0
---

Components are not made with `createReactClass` or `React.createClass`.

**Why:** `createReactClass` is a legacy package; classes and functions are what React documents and optimises.

```codelaw
match:
  kind: call_expression
  has: { field: function, regex: '^(createReactClass|React\.createClass|createClass)$' }
message: "Write this component as a function or a class"
```

## How to fix

1. Rewrite the component as a function component (or a class extending `Component`).

## Examples

### JavaScript

#### Don't

```js
const Hello = createReactClass({
  render() {
    return <div />;
  },
});
```

#### Do

```js
function Hello() {
  return <div />;
}
```

### JSX

#### Don't

```jsx
const Hello = createReactClass({
  render() {
    return <div />;
  },
});
```

#### Do

```jsx
function Hello() {
  return <div />;
}
```

### TSX

#### Don't

```tsx
const Hello = createReactClass({
  render() {
    return <div />;
  },
});
```

#### Do

```tsx
function Hello() {
  return <div />;
}
```
