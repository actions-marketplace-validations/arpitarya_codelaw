---
id: js/no-array-fill-with-reference-type
title: "No fill() with an object"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "each element gets its own value, which is the point, but code that relied on the elements being one shared object changes; a `fill` whose length is not in a `new Array(n)` or `Array(n)` before it is rewritten by hand"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-fill-with-reference-type]
since: 0.5.0
---

`.fill()` is not given an object, array, function or `new` value.

**Why:** Every slot gets the same reference, so changing one row changes them all.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^fill$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - kind: object
        - kind: array
        - kind: new_expression
        - kind: arrow_function
        - kind: function_expression
  any:
    - all: [{ any: [{ pattern: new Array($N).fill($VAL) }, { pattern: Array($N).fill($VAL) }] }, { pattern: $CALL }]
    - regex: ''
message: '`fill` puts the same object in every slot; use `Array.from({ length }, () => …)`'
fix:
  - replace: { target: $CALL, with: "Array.from({ length: $N }, () => $VAL|arrow-body)" }
```

## How to fix

1. Use `Array.from({ length: n }, () => value)`, which creates a new value for each slot.

## Examples

### JavaScript

#### Don't

```js
const grid = new Array(3).fill([]);
```

#### Do

```js
const grid = Array.from({ length: 3 }, () => []);
```

### JSX

#### Don't

```jsx
const grid = new Array(3).fill([]);
```

#### Do

```jsx
const grid = Array.from({ length: 3 }, () => []);
```

### TypeScript

#### Don't

```ts
const grid = new Array(3).fill([]);
```

#### Do

```ts
const grid = Array.from({ length: 3 }, () => []);
```

### TSX

#### Don't

```tsx
const grid = new Array(3).fill([]);
```

#### Do

```tsx
const grid = Array.from({ length: 3 }, () => []);
```
