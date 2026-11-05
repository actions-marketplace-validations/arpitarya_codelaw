---
id: js/no-inverted-boolean-check
title: "No !(a === b)"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the opposite operator is read from the token, and `!(a < b)` is not `a >= b` for NaN"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-inverted-boolean-check]
since: 0.5.0
---

A comparison is not wrapped in `!( … )`; the opposite operator is used.

**Why:** The negation is easy to miss; `!==` says it in place.

```codelaw

match:
  kind: unary_expression
  regex: '^!\s*\('
  has: { kind: parenthesized_expression, has: { kind: binary_expression, has: { field: operator, regex: '^(===|!==|==|!=|<|>|<=|>=)$' } } }
message: "Use the opposite operator instead of negating the comparison"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
if (!(a === b)) {
  run();
}
```

#### Do

```js
if (a !== b) {
  run();
}
```

### JSX

#### Don't

```jsx
if (!(a === b)) {
  run();
}
```

#### Do

```jsx
if (a !== b) {
  run();
}
```

### TypeScript

#### Don't

```ts
if (!(a === b)) {
  run();
}
```

#### Do

```ts
if (a !== b) {
  run();
}
```

### TSX

#### Don't

```tsx
if (!(a === b)) {
  run();
}
```

#### Do

```tsx
if (a !== b) {
  run();
}
```
