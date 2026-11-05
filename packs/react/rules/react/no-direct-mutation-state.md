---
id: react/no-direct-mutation-state
title: "No assigning to this.state outside the constructor"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a plain `this.state.k = e` becomes `this.setState({ k: e })`, which is batched; the reviewer confirms nothing reads the value on the next line; updater-form and nested writes carry no edit"
stability: preview
ported_from: [eslint-plugin-react/no-direct-mutation-state]
since: 0.1.0
---

A class component changes its state with `this.setState`, never by assigning to `this.state` outside
the constructor.

**Why:** A direct assignment doesn't trigger a re-render, and a later `setState` may overwrite it.

```codelaw
match:
  kind: assignment_expression
  any:
    - pattern: 'this.state.$K = $E'
      inside: { kind: expression_statement, stopBy: neighbor }
      not: { has: { field: right, regex: 'this\.state' } }
    - kind: assignment_expression
  has:
    field: left
    regex: ^this\.state\b
  not:
    inside:
      kind: method_definition
      has: { field: name, regex: ^constructor$ }
      stopBy: end
  inside: { kind: class_body, stopBy: end }
message: "Assigning to `this.state` doesn't re-render; call `this.setState`"
fix: "this.setState({ $K: $E })"
```

## How to fix

1. Replace the assignment with `this.setState(…)`; use the updater form when the new state depends on the old.

## Examples

### JavaScript

#### Don't

```js
class Menu extends React.Component {
  open() {
    this.state.open = true;
  }
}
```

#### Do

```js
class Menu extends React.Component {
  open() {
    this.setState({ open: true });
  }
}
```

### JSX

#### Don't

```jsx
class Menu extends React.Component {
  open() {
    this.state.open = true;
  }
}
```

#### Do

```jsx
class Menu extends React.Component {
  open() {
    this.setState({ open: true });
  }
}
```

### TSX

#### Don't

```tsx
class Menu extends React.Component {
  open() {
    this.state.open = true;
  }
}
```

#### Do

```tsx
class Menu extends React.Component {
  open() {
    this.setState({ open: true });
  }
}
```
