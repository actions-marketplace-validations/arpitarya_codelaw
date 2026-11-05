---
id: react/no-arrow-function-lifecycle
title: "Lifecycle methods are methods, not arrow fields"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/no-arrow-function-lifecycle]
since: 0.5.0
---

A lifecycle method (`render`, `componentDidMount`, …) of a class component is a method, not a class field holding an arrow function. Arrow fields with an expression body, or `async`, are reported but left alone by the fix's pattern.

**Why:** React calls lifecycle methods on the instance, so the arrow's bound `this` buys nothing, and a field is created again for every instance.

```codelaw
match:
  any: [{ kind: field_definition }, { kind: public_field_definition }]
  regex: '^(render|componentDidMount|componentDidUpdate|componentWillUnmount|shouldComponentUpdate|getSnapshotBeforeUpdate|componentDidCatch|componentWillMount|componentWillReceiveProps|componentWillUpdate|UNSAFE_componentWillMount|UNSAFE_componentWillReceiveProps|UNSAFE_componentWillUpdate|getChildContext)\s*='
  inside: { kind: class_body, inside: { kind: class_declaration, has: { kind: class_heritage, regex: '\b(Pure)?Component\b' } } }
  all:
    - has: { nthChild: 1, pattern: $N }
    - has:
        kind: arrow_function
        not: { regex: '^async\b' }
        all:
          - has: { field: parameters, pattern: $P }
          - has: { field: body, kind: statement_block, pattern: $B }
message: "Write `$N` as a method"
fix: "$N$P $B"
```

## Examples

### JavaScript

#### Don't

```js
class Clock extends Component {
  componentDidMount = () => {
    this.start();
  };
}
```

#### Do

```js
class Clock extends Component {
  componentDidMount() {
    this.start();
  }
}
```

#### Fixed

```js
class Clock extends Component {
  componentDidMount() {
    this.start();
  };
}
```

### JSX

#### Don't

```jsx
class Clock extends Component {
  componentDidMount = () => {
    this.start();
  };
}
```

#### Do

```jsx
class Clock extends Component {
  componentDidMount() {
    this.start();
  }
}
```

#### Fixed

```jsx
class Clock extends Component {
  componentDidMount() {
    this.start();
  };
}
```

### TSX

#### Don't

```tsx
class Clock extends Component {
  componentDidMount = () => {
    this.start();
  };
}
```

#### Do

```tsx
class Clock extends Component {
  componentDidMount() {
    this.start();
  }
}
```

#### Fixed

```tsx
class Clock extends Component {
  componentDidMount() {
    this.start();
  };
}
```
