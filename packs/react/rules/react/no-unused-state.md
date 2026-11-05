---
id: react/no-unused-state
title: "No state field a component never reads"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "whether the field is dead or should be rendered is the author's call"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react/no-unused-state]
since: 0.5.0
---

A field of a class component's state is read somewhere as `this.state.x`. A component that passes `this.state` on whole is not judged.

**Why:** Unread state re-renders the component for nothing.

```codelaw
native: react/no-unused-state
```

## How to fix

1. Remove the field, or read it.

## Examples

### JavaScript

#### Don't

```js
class A extends React.Component {
  state = { open: false };
  render() {
    return null;
  }
}
```

#### Do

```js
class A extends React.Component {
  state = { open: false };
  render() {
    return this.state.open;
  }
}
```

### JSX

#### Don't

```jsx
class A extends React.Component {
  state = { open: false };
  render() {
    return null;
  }
}
```

#### Do

```jsx
class A extends React.Component {
  state = { open: false };
  render() {
    return this.state.open;
  }
}
```

### TSX

#### Don't

```tsx
class A extends React.Component {
  state = { open: false };
  render() {
    return null;
  }
}
```

#### Do

```tsx
class A extends React.Component {
  state = { open: false };
  render() {
    return this.state.open;
  }
}
```
