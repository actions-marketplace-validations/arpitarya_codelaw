---
id: react/sort-comp
title: "Component class members are in a set order"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "moving methods carries their comments, and a class field's initialiser may read one declared before it, so the order is the author's to change"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/sort-comp]
since: 0.5.0
---

A class component lists static methods, then lifecycle members in React's order, then other methods, then `render…` helpers, then `render`.

**Why:** Every component reads the same way.

```codelaw
native: react/sort-comp
```

## How to fix

1. Move the member to its group.

## Examples

### JavaScript

#### Don't

```js
class A extends React.Component {
  render() {
    return null;
  }
  componentDidMount() {}
}
```

#### Do

```js
class A extends React.Component {
  componentDidMount() {}
  render() {
    return null;
  }
}
```

### JSX

#### Don't

```jsx
class A extends React.Component {
  render() {
    return null;
  }
  componentDidMount() {}
}
```

#### Do

```jsx
class A extends React.Component {
  componentDidMount() {}
  render() {
    return null;
  }
}
```

### TSX

#### Don't

```tsx
class A extends React.Component {
  render() {
    return null;
  }
  componentDidMount() {}
}
```

#### Do

```tsx
class A extends React.Component {
  componentDidMount() {}
  render() {
    return null;
  }
}
```
