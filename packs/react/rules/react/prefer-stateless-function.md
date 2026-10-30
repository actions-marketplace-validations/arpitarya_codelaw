---
id: react/prefer-stateless-function
title: "A class with only render is a function"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the class is rewritten as a function component"
stability: preview
ported_from: [eslint-plugin-react/prefer-stateless-function]
since: 0.5.0
---

A class component whose only member is `render`, and which reads no state, refs or `this` beyond `this.props` and `this.context`, is written as a function component.

**Why:** A function component is shorter and has no instance to reason about.

```codelaw
match:
  kind: class_declaration
  all:
    - has: { kind: class_heritage, regex: '\b(React\.)?(Pure)?Component$' }
  has:
    field: body
    all:
      - has: { kind: method_definition, has: { field: name, regex: '^render$' } }
      - not: { has: { kind: method_definition, has: { field: name, not: { regex: '^render$' } } } }
      - not: { has: { any: [{ kind: field_definition }, { kind: public_field_definition }], not: { regex: '^static\s' } } }
      - not: { has: { stopBy: end, kind: this, not: { inside: { kind: member_expression, regex: '^this\.(props|context)\b' } } } }
message: "This class only renders; write it as a function"
```

## How to fix

1. Rewrite it as `function Name(props) { return …; }`, replacing `this.props` with `props`.

## Examples

### JavaScript

#### Don't

```js
class Hello extends Component {
  render() {
    return <p>{this.props.name}</p>;
  }
}
```

#### Do

```js
function Hello(props) {
  return <p>{props.name}</p>;
}
```

### JSX

#### Don't

```jsx
class Hello extends Component {
  render() {
    return <p>{this.props.name}</p>;
  }
}
```

#### Do

```jsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```

### TSX

#### Don't

```tsx
class Hello extends Component {
  render() {
    return <p>{this.props.name}</p>;
  }
}
```

#### Do

```tsx
function Hello(props) {
  return <p>{props.name}</p>;
}
```
