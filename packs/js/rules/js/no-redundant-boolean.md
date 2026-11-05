---
id: js/no-redundant-boolean
title: "No comparison with a boolean literal"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "for a non-boolean operand the comparison and the bare value differ"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-redundant-boolean]
since: 0.5.0
---

A boolean literal is not compared with `==`/`===`, nor used as a constant operand of `||` and `&&`.

**Why:** The literal adds nothing the operand does not already say, or makes the result constant.

```codelaw

match:
  kind: binary_expression
  any:
    - pattern: "$X === true"
    - pattern: "$X === false"
    - pattern: "$X == true"
    - pattern: "$X == false"
    - pattern: "$X !== true"
    - pattern: "$X !== false"
    - pattern: "$X || true"
    - pattern: "$X && false"
message: "A boolean literal here is redundant; simplify the expression"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
if (ready === true) {
  go();
}
```

#### Do

```js
if (ready) {
  go();
}
```

### JSX

#### Don't

```jsx
if (ready === true) {
  go();
}
```

#### Do

```jsx
if (ready) {
  go();
}
```

### TypeScript

#### Don't

```ts
if (ready === true) {
  go();
}
```

#### Do

```ts
if (ready) {
  go();
}
```

### TSX

#### Don't

```tsx
if (ready === true) {
  go();
}
```

#### Do

```tsx
if (ready) {
  go();
}
```
