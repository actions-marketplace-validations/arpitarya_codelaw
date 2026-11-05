---
id: js/no-array-reduce
title: "No reduce()"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the loop needs a variable for the accumulator"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-reduce]
since: 0.5.0
---

`reduce` and `reduceRight` are not given a callback with a block body; a one-expression reducer (`(a, b) => a + b`) is allowed, as upstream allows simple ones.

**Why:** A reducer with statements hides a loop and its accumulator in a call; a `for … of` loop says the same plainly.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^(reduce|reduceRight)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - kind: arrow_function
        - kind: function_expression
        has:
          field: body
          kind: statement_block
message: A `reduce` with a block body is harder to follow than a loop
```

## How to fix

1. Declare the accumulator with `let`, loop over the array with `for … of`, and update it in the body.

## Examples

### JavaScript

#### Don't

```js
const total = items.reduce((sum, x) => {
  return sum + x.price;
}, 0);
```

#### Do

```js
let total = 0;
for (const x of items) {
  total += x.price;
}
```

### JSX

#### Don't

```jsx
const total = items.reduce((sum, x) => {
  return sum + x.price;
}, 0);
```

#### Do

```jsx
let total = 0;
for (const x of items) {
  total += x.price;
}
```

### TypeScript

#### Don't

```ts
const total = items.reduce((sum, x) => {
  return sum + x.price;
}, 0);
```

#### Do

```ts
let total = 0;
for (const x of items) {
  total += x.price;
}
```

### TSX

#### Don't

```tsx
const total = items.reduce((sum, x) => {
  return sum + x.price;
}, 0);
```

#### Do

```tsx
let total = 0;
for (const x of items) {
  total += x.price;
}
```
