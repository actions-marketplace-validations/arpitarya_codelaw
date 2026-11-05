---
id: react/prefer-function-component
title: "Components are functions"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "rewriting a class as a function with hooks restructures the component"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react/prefer-function-component]
since: 0.5.0
---

A component is a function, not a class extending `React.Component` (error boundaries excepted).

**Why:** Function components with hooks are React's current form.

```codelaw
native: react/prefer-function-component
```

## How to fix

1. Rewrite the class as a function, its state with `useState` and its lifecycle with `useEffect`.

## Examples

### JavaScript

#### Don't

```js
class A extends React.Component {
  render() {
    return null;
  }
}
```

#### Do

```js
function A() {
  return null;
}
```

### JSX

#### Don't

```jsx
class A extends React.Component {
  render() {
    return null;
  }
}
```

#### Do

```jsx
function A() {
  return null;
}
```

### TypeScript

#### Don't

```ts
class A extends React.Component {
  render() {
    return null;
  }
}
```

#### Do

```ts
function A() {
  return null;
}
```

### TSX

#### Don't

```tsx
class A extends React.Component {
  render() {
    return null;
  }
}
```

#### Do

```tsx
function A() {
  return null;
}
```
