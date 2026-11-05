---
id: js/no-negated-condition
title: "No negated condition with an else"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "`!a` and `a` are exact complements, so dropping the negation (or flipping `!==` and `!=`) and swapping the two branches keeps the behaviour"
stability: preview
ported_from: [eslint/no-negated-condition, eslint-plugin-unicorn/no-negated-condition]
since: 0.5.0
---

An `if` with an `else`, or a conditional expression, does not test a negated condition.

**Why:** `if (!a) x else y` reads as a double negative; `if (a) y else x` says the same directly.

```codelaw
match:
  any:
  - kind: if_statement
    all:
    - has: { field: condition, has: { kind: unary_expression, regex: ^!, pattern: $NI1, has: { field: argument, pattern: $XI1 } } }
    - has: { field: consequence, pattern: $CI1 }
    - has:
        field: alternative
        not: { has: { kind: if_statement } }
        has: { pattern: $AI1 }
  - kind: if_statement
    all:
    - has: { field: condition, has: { kind: binary_expression, all: [{ pattern: "$LI2 !== $RI2" }, { pattern: $NI2 }] } }
    - has: { field: consequence, pattern: $CI2 }
    - has:
        field: alternative
        not: { has: { kind: if_statement } }
        has: { pattern: $AI2 }
  - kind: if_statement
    all:
    - has: { field: condition, has: { kind: binary_expression, all: [{ pattern: "$LI3 != $RI3" }, { pattern: $NI3 }] } }
    - has: { field: consequence, pattern: $CI3 }
    - has:
        field: alternative
        not: { has: { kind: if_statement } }
        has: { pattern: $AI3 }
  - kind: ternary_expression
    all:
    - has: { field: condition, kind: unary_expression, regex: ^!, pattern: $NT1, has: { field: argument, pattern: $XT1 } }
    - has: { field: consequence, pattern: $CT1 }
    - has: { field: alternative, pattern: $AT1 }
  - kind: ternary_expression
    all:
    - has: { field: condition, kind: binary_expression, all: [{ pattern: "$LT2 !== $RT2" }, { pattern: $NT2 }] }
    - has: { field: consequence, pattern: $CT2 }
    - has: { field: alternative, pattern: $AT2 }
  - kind: ternary_expression
    all:
    - has: { field: condition, kind: binary_expression, all: [{ pattern: "$LT3 != $RT3" }, { pattern: $NT3 }] }
    - has: { field: consequence, pattern: $CT3 }
    - has: { field: alternative, pattern: $AT3 }
message: Unexpected negated condition; swap the branches
fix:
  - replace: { target: $NI1, with: "$XI1" }
  - replace: { target: $CI1, with: "$AI1" }
  - replace: { target: $AI1, with: "$CI1" }
  - replace: { target: $NI2, with: "$LI2 === $RI2" }
  - replace: { target: $CI2, with: "$AI2" }
  - replace: { target: $AI2, with: "$CI2" }
  - replace: { target: $NI3, with: "$LI3 == $RI3" }
  - replace: { target: $CI3, with: "$AI3" }
  - replace: { target: $AI3, with: "$CI3" }
  - replace: { target: $NT1, with: "$XT1" }
  - replace: { target: $CT1, with: "$AT1" }
  - replace: { target: $AT1, with: "$CT1" }
  - replace: { target: $NT2, with: "$LT2 === $RT2" }
  - replace: { target: $CT2, with: "$AT2" }
  - replace: { target: $AT2, with: "$CT2" }
  - replace: { target: $NT3, with: "$LT3 == $RT3" }
  - replace: { target: $CT3, with: "$AT3" }
  - replace: { target: $AT3, with: "$CT3" }
```

## How to fix

1. Remove the negation and swap the two branches.

## Examples

### JavaScript

#### Don't

```js
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Fixed

```js
if (ok) {
  pass();
} else {
  fail();
}
```

#### Do

```js
if (ok) {
  pass();
} else {
  fail();
}
```

### JSX

#### Don't

```jsx
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Fixed

```jsx
if (ok) {
  pass();
} else {
  fail();
}
```

#### Do

```jsx
if (ok) {
  pass();
} else {
  fail();
}
```

### TypeScript

#### Don't

```ts
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Fixed

```ts
if (ok) {
  pass();
} else {
  fail();
}
```

#### Do

```ts
if (ok) {
  pass();
} else {
  fail();
}
```

### TSX

#### Don't

```tsx
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Fixed

```tsx
if (ok) {
  pass();
} else {
  fail();
}
```

#### Do

```tsx
if (ok) {
  pass();
} else {
  fail();
}
```
