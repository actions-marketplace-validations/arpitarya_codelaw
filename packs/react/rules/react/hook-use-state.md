---
id: react/hook-use-state
title: "useState is destructured as [value, setValue]"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the names are the author's; a template cannot recase the value's name into the setter's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/hook-use-state]
since: 0.5.0
---

The result of `useState` is destructured into exactly two names, the second starting with `set` (`const [open, setOpen] = useState(false)`). That the setter repeats the value's name is not checked yet.

**Why:** The pair naming makes the setter findable from the value, and the reverse.

```codelaw
match:
  kind: variable_declarator
  has: { field: value, kind: call_expression, has: { field: function, regex: '^(React\.)?useState$' } }
  not:
    has:
      field: name
      kind: array_pattern
      all:
        - has: { nthChild: 2, kind: identifier, regex: '^set[A-Z]' }
        - not: { has: { nthChild: 3 } }
message: "Destructure `useState` as `[value, setValue]`"
```

## How to fix

1. Write `const [thing, setThing] = useState(…)`.

## Examples

### JavaScript

#### Don't

```js
function T() {
  const state = useState(false);
  return state[0];
}
```

#### Do

```js
function T() {
  const [open, setOpen] = useState(false);
  return open;
}
```

### JSX

#### Don't

```jsx
function T() {
  const state = useState(false);
  return state[0];
}
```

#### Do

```jsx
function T() {
  const [open, setOpen] = useState(false);
  return open;
}
```

### TSX

#### Don't

```tsx
function T() {
  const state = useState(false);
  return state[0];
}
```

#### Do

```tsx
function T() {
  const [open, setOpen] = useState(false);
  return open;
}
```
