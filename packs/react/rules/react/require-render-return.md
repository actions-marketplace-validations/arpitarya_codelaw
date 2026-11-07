---
id: react/require-render-return
title: "A class component's render() returns"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a `render()` whose last statement is a bare JSX expression gets `return` in front of it, so it draws what it builds; any other `render()` is manual"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/require-render-return]
since: 0.1.0
---

The `render()` method of a class component returns a value on every path.

**Why:** A `render()` that falls off its end returns `undefined`, which is almost always a forgotten
`return`.

```codelaw
native: react/require-render-return
```

## How to fix

1. Return the element, or `null` to render nothing, on every path.

## Examples

### JavaScript

#### Don't

```js
class Title extends React.Component {
  render() {
    <h1>{this.props.text}</h1>;
  }
}
```

#### Do

```js
class Title extends React.Component {
  render() {
    return <h1>{this.props.text}</h1>;
  }
}
```

### JSX

#### Don't

```jsx
class Title extends React.Component {
  render() {
    <h1>{this.props.text}</h1>;
  }
}
```

#### Do

```jsx
class Title extends React.Component {
  render() {
    return <h1>{this.props.text}</h1>;
  }
}
```

### TSX

#### Don't

```tsx
class Title extends React.Component {
  render() {
    <h1>{this.props.text}</h1>;
  }
}
```

#### Do

```tsx
class Title extends React.Component {
  render() {
    return <h1>{this.props.text}</h1>;
  }
}
```
