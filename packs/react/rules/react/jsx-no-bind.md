---
id: react/jsx-no-bind
title: "No new function in a JSX prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the function moves out of render, into a method or a `useCallback` with its dependencies"
stability: preview
ported_from: [eslint-plugin-react/jsx-no-bind]
since: 0.5.0
---

A JSX prop is not given a function created in place: an arrow function, a function expression, or a `.bind(…)` call.

**Why:** A new function on every render gives a memoised child a new prop each time, so it re-renders anyway.

```codelaw
match:
  kind: jsx_expression
  inside: { kind: jsx_attribute, not: { regex: '^(ref|key)\b' } }
  has:
    any:
      - kind: arrow_function
      - kind: function_expression
      - { kind: call_expression, has: { field: function, kind: member_expression, regex: '\.bind$' } }
message: "A function created in a JSX prop is new on every render"
```

## How to fix

1. Define the handler once: a class method, or a `useCallback` in a function component.
2. Pass it by name.

## Examples

### JavaScript

#### Don't

```js
const r = <Row onClick={() => select(id)} />;
```

#### Do

```js
const r = <Row onClick={handleSelect} />;
```

### JSX

#### Don't

```jsx
const r = <Row onClick={() => select(id)} />;
```

#### Do

```jsx
const r = <Row onClick={handleSelect} />;
```

### TSX

#### Don't

```tsx
const r = <Row onClick={() => select(id)} />;
```

#### Do

```tsx
const r = <Row onClick={handleSelect} />;
```
