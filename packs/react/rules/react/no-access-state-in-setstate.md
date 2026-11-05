---
id: react/no-access-state-in-setstate
title: "setState does not read this.state"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the update becomes an updater function, `setState((state) => …)`, which reads its own state"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-access-state-in-setstate]
since: 0.5.0
---

An object passed to `this.setState` is not built from `this.state`.

**Why:** State updates are batched, so `this.state` may already be stale: two quick clicks of `count: this.state.count + 1` add one.

```codelaw
match:
  kind: member_expression
  regex: '^this\.state\b'
  inside:
    stopBy: { any: [{ kind: function_declaration }, { kind: function_expression }, { kind: arrow_function }] }
    kind: arguments
    inside: { kind: call_expression, has: { field: function, regex: '^this\.setState$' } }
message: "Don't read `this.state` inside `setState`; pass an updater function"
```

## How to fix

1. Pass a function: `this.setState((state) => ({ count: state.count + 1 }))`.

## Examples

### JavaScript

#### Don't

```js
class C extends Component {
  inc() {
    this.setState({ count: this.state.count + 1 });
  }
}
```

#### Do

```js
class C extends Component {
  inc() {
    this.setState((state) => ({ count: state.count + 1 }));
  }
}
```

### JSX

#### Don't

```jsx
class C extends Component {
  inc() {
    this.setState({ count: this.state.count + 1 });
  }
}
```

#### Do

```jsx
class C extends Component {
  inc() {
    this.setState((state) => ({ count: state.count + 1 }));
  }
}
```

### TSX

#### Don't

```tsx
class C extends Component {
  inc() {
    this.setState({ count: this.state.count + 1 });
  }
}
```

#### Do

```tsx
class C extends Component {
  inc() {
    this.setState((state) => ({ count: state.count + 1 }));
  }
}
```
