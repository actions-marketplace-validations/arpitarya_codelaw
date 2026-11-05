---
id: js/no-compare-neg-zero
title: "No comparisons against -0"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`0` keeps what the code does today, `Object.is` tests for -0 as the code seems to intend; the author picks"
stability: preview
ported_from: [eslint/no-compare-neg-zero]
since: 0.1.0
---

Code doesn't compare a value with `-0` using `==`, `===`, `<` or the other comparison operators.

**Why:** These operators treat `-0` and `0` as equal, so the comparison doesn't test what it says.

```codelaw
match:
  kind: binary_expression
  all:
    - any:
        - has: { field: right, all: [{ pattern: $RZ }, { pattern: "-0" }] }
        - has: { field: left, all: [{ pattern: $LZ }, { pattern: "-0" }] }
    - has:
        field: operator
        regex: ^(==|===|!=|!==|<|>|<=|>=)$
    - any:
        - all:
            - { pattern: $EQ }
            - any: [{ pattern: "$X === -0" }, { pattern: "-0 === $X" }, { pattern: "$X == -0" }, { pattern: "-0 == $X" }]
        - all:
            - { pattern: $NE }
            - any: [{ pattern: "$X !== -0" }, { pattern: "-0 !== $X" }, { pattern: "$X != -0" }, { pattern: "-0 != $X" }]
        - { kind: binary_expression }
message: "Comparing with -0 is the same as comparing with 0; use Object.is(x, -0) to test for -0"
fix:
  choices:
    - label: compare with 0
      default: true
      fix:
        - replace: { target: $RZ, with: "0" }
        - replace: { target: $LZ, with: "0" }
    - label: use Object.is
      fix:
        - replace: { target: $EQ, with: "Object.is($X, -0)" }
        - replace: { target: $NE, with: "!Object.is($X, -0)" }
```

## How to fix

1. To test for negative zero, use `Object.is(x, -0)`.
2. To test for zero of either sign, compare with `0`.

## Examples

### JavaScript

#### Don't

```js
if (x === -0) {
  flip();
}
```

#### Fixed (compare with 0)

```js
if (x === 0) {
  flip();
}
```

#### Fixed (use Object.is)

```js
if (Object.is(x, -0)) {
  flip();
}
```

#### Do

```js
if (Object.is(x, -0)) {
  flip();
}
```

### JSX

#### Don't

```jsx
if (x === -0) {
  flip();
}
```

#### Fixed (compare with 0)

```jsx
if (x === 0) {
  flip();
}
```

#### Fixed (use Object.is)

```jsx
if (Object.is(x, -0)) {
  flip();
}
```

#### Do

```jsx
if (Object.is(x, -0)) {
  flip();
}
```

### TypeScript

#### Don't

```ts
if (x === -0) {
  flip();
}
```

#### Fixed (compare with 0)

```ts
if (x === 0) {
  flip();
}
```

#### Fixed (use Object.is)

```ts
if (Object.is(x, -0)) {
  flip();
}
```

#### Do

```ts
if (Object.is(x, -0)) {
  flip();
}
```

### TSX

#### Don't

```tsx
if (x === -0) {
  flip();
}
```

#### Fixed (compare with 0)

```tsx
if (x === 0) {
  flip();
}
```

#### Fixed (use Object.is)

```tsx
if (Object.is(x, -0)) {
  flip();
}
```

#### Do

```tsx
if (Object.is(x, -0)) {
  flip();
}
```
