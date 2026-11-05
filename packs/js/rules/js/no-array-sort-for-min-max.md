---
id: js/no-array-sort-for-min-max
title: "Math.min/max rather than sorting for one value"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of `Math.min` and `Math.max` the comparator means is read from the comparator, which a template cannot do"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-sort-for-min-max]
since: 0.5.0
---

The first element of a sorted copy is not read just to find the smallest or largest value.

**Why:** Sorting does far more work than a single pass, and `.sort()` also reorders the original.

```codelaw

match:
  kind: subscript_expression
  any:
    - pattern: $A.sort($$$X)[0]
    - pattern: $A.toSorted($$$X)[0]
message: "Sorting to read one end is O(n log n); use `Math.min`/`Math.max` or a loop"
```

## How to fix

1. If the comparator sorts ascending, use `Math.min(...values)`; descending, `Math.max(...values)`. For objects, find the extreme with a `reduce` or a loop.

## Examples

### JavaScript

#### Don't

```js
const first = values.toSorted((a, b) => a - b)[0];
```

#### Do

```js
const first = Math.min(...values);
```

### JSX

#### Don't

```jsx
const first = values.toSorted((a, b) => a - b)[0];
```

#### Do

```jsx
const first = Math.min(...values);
```

### TypeScript

#### Don't

```ts
const first = values.toSorted((a, b) => a - b)[0];
```

#### Do

```ts
const first = Math.min(...values);
```

### TSX

#### Don't

```tsx
const first = values.toSorted((a, b) => a - b)[0];
```

#### Do

```tsx
const first = Math.min(...values);
```
