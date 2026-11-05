---
id: react/no-unused-class-component-methods
title: "Every class component method is used"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "deleting an unused method is safe unless something outside the class calls it through a ref"
stability: preview
ported_from: [eslint-plugin-react/no-unused-class-component-methods]
since: 0.5.0
---

A method of a class component that is not a lifecycle method is used somewhere in the class (`this.method`).

**Why:** An unused method is dead code that still has to be read and maintained.

```codelaw
match:
  kind: method_definition
  not: { regex: '^(static|get|set|async\s+get)\s' }
  all:
    - has: { field: name, kind: property_identifier, not: { regex: '^(render|componentDidMount|componentDidUpdate|componentWillUnmount|shouldComponentUpdate|getSnapshotBeforeUpdate|componentDidCatch|componentWillMount|componentWillReceiveProps|componentWillUpdate|UNSAFE_componentWillMount|UNSAFE_componentWillReceiveProps|UNSAFE_componentWillUpdate|getChildContext|constructor|getSnapshotBeforeUpdate|getDerivedStateFromProps|getDerivedStateFromError|UNSAFE_\w+)$' }, pattern: $M }
    - inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } }
    - not: { inside: { kind: class_body, has: { stopBy: end, kind: member_expression, all: [{ has: { field: object, kind: this } }, { has: { field: property, pattern: $M } }] } } }
    - not: { inside: { kind: class_body, has: { stopBy: end, kind: shorthand_property_identifier_pattern, pattern: $M } } }
message: "`$M` is never used"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
class A extends Component {
  helper() {
    return 1;
  }
  render() {
    return null;
  }
}
```

#### Do

```js
class A extends Component {
  render() {
    return null;
  }
}
```

### JSX

#### Don't

```jsx
class A extends Component {
  helper() {
    return 1;
  }
  render() {
    return null;
  }
}
```

#### Do

```jsx
class A extends Component {
  render() {
    return null;
  }
}
```

### TSX

#### Don't

```tsx
class A extends Component {
  helper() {
    return 1;
  }
  render() {
    return null;
  }
}
```

#### Do

```tsx
class A extends Component {
  render() {
    return null;
  }
}
```
