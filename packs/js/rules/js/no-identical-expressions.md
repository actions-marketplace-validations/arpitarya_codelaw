---
id: js/no-identical-expressions
title: "No identical operands"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which operand was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-identical-expressions]
since: 0.5.0
---

The two operands of a comparison, logical, subtraction, division or xor operator are not the same expression. Operands containing a call are left alone, and `x !== x` is left to `js/use-isnan`'s NaN idiom only when the operand is a number literal.

**Why:** The result is constant (`a - a` is 0, `a === a` is true but for NaN), so one operand is almost certainly a typo.

```codelaw
match:
  kind: binary_expression
  any:
  - pattern: $A == $A
  - pattern: $A === $A
  - pattern: $A != $A
  - pattern: $A !== $A
  - pattern: $A && $A
  - pattern: $A || $A
  - pattern: $A - $A
  - pattern: $A / $A
  - pattern: $A < $A
  - pattern: $A > $A
  - pattern: $A <= $A
  - pattern: $A >= $A
  - pattern: $A ^ $A
  not:
    has:
      stopBy: end
      any:
      - kind: call_expression
      - kind: update_expression
      - kind: new_expression
  all:
  - not:
      has:
        field: left
        kind: number
message: Both sides of this operator are the same; one was probably meant to be different
```

## How to fix

1. Correct the operand that was meant to be different, or replace the expression with its constant result.

## Examples

### JavaScript

#### Don't

```js
if (a.x === a.x) {
  run();
}
```

#### Do

```js
if (a.x === b.x) {
  run();
}
```

### JSX

#### Don't

```jsx
if (a.x === a.x) {
  run();
}
```

#### Do

```jsx
if (a.x === b.x) {
  run();
}
```

### TypeScript

#### Don't

```ts
if (a.x === a.x) {
  run();
}
```

#### Do

```ts
if (a.x === b.x) {
  run();
}
```

### TSX

#### Don't

```tsx
if (a.x === a.x) {
  run();
}
```

#### Do

```tsx
if (a.x === b.x) {
  run();
}
```
