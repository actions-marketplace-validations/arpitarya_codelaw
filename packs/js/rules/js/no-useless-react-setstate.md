---
id: js/no-useless-react-setstate
title: "No state setter called with its own state"
languages: [jsx, tsx]
fix: manual
fix_reason: "the value the setter was meant to get is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-useless-react-setstate]
since: 0.5.0
---

A `useState` setter is not called with the state variable it sets: `setCount(count)` changes nothing.

**Why:** React bails out when the value is the same, so the call is a slip for another value.

```codelaw
match:
  pattern: "$SET($V)"
  inside:
    stopBy: end
    kind: statement_block
    has:
      any:
        - pattern: "const [$V, $SET] = useState($$$I);"
        - pattern: "const [$V, $SET] = React.useState($$$I);"
message: "`$SET($V)` sets the state to what it already is"
```

## How to fix

1. Pass the value the state should become, often computed from the current one: `$SET(c => c + 1)`.

## Examples

### JSX

#### Don't

```jsx
function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count)}>{count}</button>;
}
```

#### Do

```jsx
function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```


### TSX

#### Don't

```tsx
function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count)}>{count}</button>;
}
```

#### Do

```tsx
function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```
