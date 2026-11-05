---
id: react/no-did-mount-set-state
title: "No setState in componentDidMount"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the state is derived in render, or the update moves to a handler"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-did-mount-set-state]
since: 0.5.0
---

A class component does not call `this.setState` directly in `componentDidMount`. A call inside a callback defined there (a subscription, a timer) is fine.

**Why:** A state update in `componentDidMount` triggers a second render before the browser paints, and is usually state that could be computed.

```codelaw
match:
  pattern: this.setState($$$A)
  inside:
    stopBy: { any: [{ kind: function_declaration }, { kind: function_expression }, { kind: arrow_function }, { kind: method_definition }] }
    kind: method_definition
    has: { field: name, regex: '^componentDidMount$' }
message: "`this.setState` in `componentDidMount` renders twice"
```

## How to fix

1. Compute the value in `render` from props and state instead of storing it, or move the update into an event handler.

## Examples

### JavaScript

#### Don't

```js
class Panel extends Component {
  componentDidMount() {
    this.setState({ ready: true });
  }
}
```

#### Do

```js
class Panel extends Component {
  componentDidMount() {
    this.timer = setTimeout(() => this.setState({ ready: true }), 100);
  }
}
```

### JSX

#### Don't

```jsx
class Panel extends Component {
  componentDidMount() {
    this.setState({ ready: true });
  }
}
```

#### Do

```jsx
class Panel extends Component {
  componentDidMount() {
    this.timer = setTimeout(() => this.setState({ ready: true }), 100);
  }
}
```

### TSX

#### Don't

```tsx
class Panel extends Component {
  componentDidMount() {
    this.setState({ ready: true });
  }
}
```

#### Do

```tsx
class Panel extends Component {
  componentDidMount() {
    this.timer = setTimeout(() => this.setState({ ready: true }), 100);
  }
}
```
