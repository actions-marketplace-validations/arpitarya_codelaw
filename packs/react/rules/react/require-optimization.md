---
id: react/require-optimization
title: "Class components say when to skip a render"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: extending `PureComponent` or writing `shouldComponentUpdate` changes when the component renders, which the author has to confirm"
stability: preview
ported_from: [eslint-plugin-react/require-optimization]
since: 0.5.0
---

A class extending `Component` defines `shouldComponentUpdate`, or extends `PureComponent` instead.

**Why:** Without either, the component re-renders whenever its parent does, changed props or not.

```codelaw
match:
  kind: class_declaration
  has: { kind: class_heritage, regex: '\b(React\.)?Component$' }
  not: { has: { field: body, has: { kind: method_definition, has: { field: name, regex: '^shouldComponentUpdate$' } } } }
message: "Extend `PureComponent` or define `shouldComponentUpdate`"
```

## How to fix

1. Extend `PureComponent` if the props compare shallowly, or write `shouldComponentUpdate`.

## Examples

### JavaScript

#### Don't

```js
class Row extends Component {
  render() {
    return null;
  }
}
```

#### Do

```js
class Row extends PureComponent {
  render() {
    return null;
  }
}
```

### JSX

#### Don't

```jsx
class Row extends Component {
  render() {
    return null;
  }
}
```

#### Do

```jsx
class Row extends PureComponent {
  render() {
    return null;
  }
}
```

### TSX

#### Don't

```tsx
class Row extends Component {
  render() {
    return null;
  }
}
```

#### Do

```tsx
class Row extends PureComponent {
  render() {
    return null;
  }
}
```
