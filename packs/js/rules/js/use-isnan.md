---
id: js/use-isnan
title: "Test for NaN with Number.isNaN"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the comparison is constant today, so the fix changes behaviour by design; the operator picks the negation and the reviewer confirms"
stability: preview
ported_from: [eslint/use-isnan]
since: 0.1.0
---

Code tests for `NaN` with `Number.isNaN(x)`, never by comparing with `NaN`.

**Why:** `NaN` is not equal to anything, itself included, so `x === NaN` is always false.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^(==|===|!=|!==|<|>|<=|>=)$
  any:
    - pattern: $PLN
      has: { field: operator, regex: '^(==|===)$' }
      all:
        - has: { field: left, regex: '^(Number\.)?NaN$' }
        - has: { field: right, pattern: $PLX }
    - pattern: $PRN
      has: { field: operator, regex: '^(==|===)$' }
      all:
        - has: { field: right, regex: '^(Number\.)?NaN$' }
        - has: { field: left, pattern: $PRX }
    - pattern: $NLN
      has: { field: operator, regex: '^(!=|!==)$' }
      all:
        - has: { field: left, regex: '^(Number\.)?NaN$' }
        - has: { field: right, pattern: $NLX }
    - pattern: $NRN
      has: { field: operator, regex: '^(!=|!==)$' }
      all:
        - has: { field: right, regex: '^(Number\.)?NaN$' }
        - has: { field: left, pattern: $NRX }
    - has: { field: left, regex: '^(Number\.)?NaN$' }
    - has: { field: right, regex: '^(Number\.)?NaN$' }
message: "Nothing equals NaN, not even NaN; use Number.isNaN(x)"
fix:
  - replace: { target: $PLN, with: "Number.isNaN($PLX)" }
  - replace: { target: $PRN, with: "Number.isNaN($PRX)" }
  - replace: { target: $NLN, with: "!Number.isNaN($NLX)" }
  - replace: { target: $NRN, with: "!Number.isNaN($NRX)" }
```

## How to fix

1. Check the branch that ran on a comparison with `NaN` (always false) is the one meant, then keep the `Number.isNaN` form.

## Examples

### JavaScript

#### Don't

```js
if (price === NaN) {
  reset();
}
```

#### Fixed

```js
if (Number.isNaN(price)) {
  reset();
}
```

#### Do

```js
if (Number.isNaN(price)) {
  reset();
}
```

### JSX

#### Don't

```jsx
if (price === NaN) {
  reset();
}
```

#### Fixed

```jsx
if (Number.isNaN(price)) {
  reset();
}
```

#### Do

```jsx
if (Number.isNaN(price)) {
  reset();
}
```

### TypeScript

#### Don't

```ts
if (price === NaN) {
  reset();
}
```

#### Fixed

```ts
if (Number.isNaN(price)) {
  reset();
}
```

#### Do

```ts
if (Number.isNaN(price)) {
  reset();
}
```

### TSX

#### Don't

```tsx
if (price === NaN) {
  reset();
}
```

#### Fixed

```tsx
if (Number.isNaN(price)) {
  reset();
}
```

#### Do

```tsx
if (Number.isNaN(price)) {
  reset();
}
```
