---
id: js/no-return-assign
title: "No assignment in a return"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "re-reading a plain identifier after assigning it gives the assigned value, so `return x = e` becomes `x = e; return x;` with the same behaviour; member targets (a getter may run) report suggest"
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
      pattern: $S1
      has:
        kind: assignment_expression
        all:
          - has: { field: left, kind: identifier, pattern: $X1 }
          - has: { field: right, pattern: $E1 }
    - kind: assignment_expression
      pattern: $T2
      inside: { kind: arrow_function, field: body }
      all:
        - has: { field: left, kind: identifier, pattern: $X2 }
        - has: { field: right, pattern: $E2 }
    - kind: return_statement
      has: { kind: assignment_expression }
    - kind: assignment_expression
      inside: { kind: arrow_function, field: body }
message: "Do not return an assignment; compare with `===`, or assign first"
auto_when:
  any:
    - kind: return_statement
      has: { kind: assignment_expression, has: { field: left, kind: identifier } }
    - kind: assignment_expression
      has: { field: left, kind: identifier }
fix:
  - replace: { target: $S1, with: "$X1 = $E1; return $X1;" }
  - replace: { target: $T2, with: "{ $X2 = $E2; return $X2; }" }
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

#### Fixed

```js
function set(v) {
  value = v; return value;
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

#### Fixed

```jsx
function set(v) {
  value = v; return value;
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

#### Fixed

```ts
function set(v) {
  value = v; return value;
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

#### Fixed

```tsx
function set(v) {
  value = v; return value;
}
```

#### Do

```tsx
function set(v) {
  value = v;
  return value;
}
```
