---
id: js/no-self-compare
title: "No comparing a value with itself"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "an identifier compared with itself by equality is a NaN test, so `x !== x` becomes `Number.isNaN(x)` and `x === x` becomes `!Number.isNaN(x)` with the same result; member operands and relational forms carry no edit"
stability: preview
ported_from: [eslint/no-self-compare]
since: 0.5.0
---

An expression is not compared with itself.

**Why:** `x === x` is true except for `NaN` — use `Number.isNaN` for that — and is otherwise a typo.

```codelaw
match:
  any:
    - kind: binary_expression
      all:
        - pattern: "$A1 !== $A1"
        - pattern: $M1
        - has: { field: left, kind: identifier }
    - kind: binary_expression
      all:
        - pattern: "$A2 != $A2"
        - pattern: $M2
        - has: { field: left, kind: identifier }
    - kind: binary_expression
      all:
        - pattern: "$A3 === $A3"
        - pattern: $M3
        - has: { field: left, kind: identifier }
    - kind: binary_expression
      all:
        - pattern: "$A4 == $A4"
        - pattern: $M4
        - has: { field: left, kind: identifier }
    - pattern: $A === $A
    - pattern: $A == $A
    - pattern: $A !== $A
    - pattern: $A != $A
    - pattern: $A < $A
    - pattern: $A > $A
    - pattern: $A <= $A
    - pattern: $A >= $A
message: "Comparing `$A` with itself"
fix:
  - replace: { target: $M1, with: "Number.isNaN($A1)" }
  - replace: { target: $M2, with: "Number.isNaN($A2)" }
  - replace: { target: $M3, with: "!Number.isNaN($A3)" }
  - replace: { target: $M4, with: "!Number.isNaN($A4)" }
```

## How to fix

1. Compare with the value meant, or use `Number.isNaN(x)` to test for `NaN`.

## Examples

### JavaScript

#### Don't

```js
if (x !== x) skip();
```

#### Do

```js
if (Number.isNaN(x)) skip();
```

### JSX

#### Don't

```jsx
if (x !== x) skip();
```

#### Do

```jsx
if (Number.isNaN(x)) skip();
```

### TypeScript

#### Don't

```ts
if (x !== x) skip();
```

#### Do

```ts
if (Number.isNaN(x)) skip();
```

### TSX

#### Don't

```tsx
if (x !== x) skip();
```

#### Do

```tsx
if (Number.isNaN(x)) skip();
```
