---
id: react/no-redundant-should-component-update
title: "No shouldComponentUpdate on a PureComponent"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing the method falls back to PureComponent's shallow comparison, which may re-render where the custom check did not"
stability: preview
ported_from: [eslint-plugin-react/no-redundant-should-component-update]
since: 0.5.0
---

A class extending `PureComponent` does not define `shouldComponentUpdate`.

**Why:** `PureComponent` exists to provide `shouldComponentUpdate`; defining it again overrides that, and the base class is misleading.

```codelaw
match:
  kind: method_definition
  has: { field: name, regex: '^shouldComponentUpdate$' }
  inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\bPureComponent\b' } } }
message: "A `PureComponent` already has `shouldComponentUpdate`"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
class Row extends PureComponent {
  shouldComponentUpdate(next) {
    return next.id !== this.props.id;
  }
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
class Row extends PureComponent {
  shouldComponentUpdate(next) {
    return next.id !== this.props.id;
  }
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
class Row extends PureComponent {
  shouldComponentUpdate(next) {
    return next.id !== this.props.id;
  }
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
