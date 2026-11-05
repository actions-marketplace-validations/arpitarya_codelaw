---
id: react/no-unsafe
title: "No UNSAFE_ lifecycle methods"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: each legacy method has a different modern replacement"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-unsafe]
since: 0.5.0
---

Class components do not define `UNSAFE_componentWillMount`, `UNSAFE_componentWillReceiveProps` or `UNSAFE_componentWillUpdate`.

**Why:** These methods misbehave under concurrent rendering, which is why React renamed them.

```codelaw
match:
  kind: method_definition
  has: { field: name, regex: '^UNSAFE_component(WillMount|WillReceiveProps|WillUpdate)$', pattern: $N }
message: "`$N` is unsafe under concurrent rendering"
```

## How to fix

1. `UNSAFE_componentWillMount`: move the code to the constructor or `componentDidMount`.
2. `UNSAFE_componentWillReceiveProps`: use `getDerivedStateFromProps` or a key.
3. `UNSAFE_componentWillUpdate`: use `getSnapshotBeforeUpdate` with `componentDidUpdate`.

## Examples

### JavaScript

#### Don't

```js
class A extends Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```js
class A extends Component {
  componentDidMount() {
    this.load();
  }
}
```

### JSX

#### Don't

```jsx
class A extends Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```jsx
class A extends Component {
  componentDidMount() {
    this.load();
  }
}
```

### TSX

#### Don't

```tsx
class A extends Component {
  UNSAFE_componentWillMount() {
    this.load();
  }
}
```

#### Do

```tsx
class A extends Component {
  componentDidMount() {
    this.load();
  }
}
```
