---
id: js/no-confusing-array-with
title: "No with() at an index that can be out of range"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what should happen on an empty array, or which index was meant, is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-confusing-array-with]
since: 0.5.0
---

`Array#with()` throws a `RangeError` for an index outside the array. `array.with(array.length, value)` is always outside it, and a negative literal index throws on an array shorter than it.

**Why:** Both read as harmless and throw at run time on some input.

```codelaw
match:
  kind: call_expression
  any:
    - pattern: "$A.with($A.length, $V)"
    - all:
        - pattern: "$A.with($I, $V)"
        - has: { field: arguments, has: { nthChild: 1, kind: unary_expression, regex: '^-\s*\d' } }
message: "`with()` throws for this index on some arrays"
```

## How to fix

1. To append, write `[...array, value]`.
2. For the last element, guard the empty case: `array.length === 0 ? array : array.with(array.length - 1, value)`.

## Examples

### JavaScript

#### Don't

```js
const next = items.with(items.length, item);
```

#### Do

```js
const next = [...items, item];
```


### JSX

#### Don't

```jsx
const next = items.with(items.length, item);
```

#### Do

```jsx
const next = [...items, item];
```


### TypeScript

#### Don't

```ts
const next = items.with(items.length, item);
```

#### Do

```ts
const next = [...items, item];
```


### TSX

#### Don't

```tsx
const next = items.with(items.length, item);
```

#### Do

```tsx
const next = [...items, item];
```
