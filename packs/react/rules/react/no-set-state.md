---
id: react/no-set-state
title: "No this.setState"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the state moves to a store or to a parent, which is an architecture choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-set-state]
since: 0.5.0
---

Components do not call `this.setState`; state lives in a store or a parent.

**Why:** For projects whose state is kept outside components (Redux, a state machine), a local `setState` is state that escaped.

```codelaw
match:
  pattern: this.setState($$$A)
message: "Keep state outside the component; don't call `this.setState`"
```

## How to fix

1. Move the state to the project's store (or a parent component) and pass it in as props.

## Examples

### JavaScript

#### Don't

```js
class Toggle extends Component {
  flip() {
    this.setState({ on: true });
  }
}
```

#### Do

```js
class Toggle extends Component {
  flip() {
    this.props.onFlip();
  }
}
```

### JSX

#### Don't

```jsx
class Toggle extends Component {
  flip() {
    this.setState({ on: true });
  }
}
```

#### Do

```jsx
class Toggle extends Component {
  flip() {
    this.props.onFlip();
  }
}
```

### TSX

#### Don't

```tsx
class Toggle extends Component {
  flip() {
    this.setState({ on: true });
  }
}
```

#### Do

```tsx
class Toggle extends Component {
  flip() {
    this.props.onFlip();
  }
}
```
