---
id: js/no-return-assign
title: "No assignment in a return"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "it may be a comparison or an assignment; the author decides"
stability: preview
ported_from: [eslint/no-return-assign]
since: 0.5.0
---

A `return` or an arrow's body is not an assignment, unless wrapped in parentheses.

**Why:** `return a = b` is usually a typo for `return a === b`.

```codelaw
match:
  any:
    - kind: return_statement
      has: { kind: assignment_expression }
    - kind: assignment_expression
      inside: { kind: arrow_function, field: body }
message: "Do not return an assignment; compare with `===`, or assign first"
```

## How to fix

1. Use `===` if a comparison was meant; otherwise assign on its own line and return the value.

## Examples

### JavaScript

#### Don't

```js
function set(v) {
  return value = v;
}
```

#### Do

```js
function set(v) {
  value = v;
  return value;
}
```

### JSX

#### Don't

```jsx
function set(v) {
  return value = v;
}
```

#### Do

```jsx
function set(v) {
  value = v;
  return value;
}
```

### TypeScript

#### Don't

```ts
function set(v) {
  return value = v;
}
```

#### Do

```ts
function set(v) {
  value = v;
  return value;
}
```

### TSX

#### Don't

```tsx
function set(v) {
  return value = v;
}
```

#### Do

```tsx
function set(v) {
  value = v;
  return value;
}
```
