---
id: js/no-accumulating-spread
title: "No spread of the accumulator in reduce"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "pushing onto the accumulator changes the callback's shape; the author rewrites it"
stability: preview
ported_from: [biome/performance/noAccumulatingSpread]
since: 0.5.0
---

A `reduce` callback does not build a new array or object by spreading its accumulator on every step.

**Why:** Each step copies everything accumulated so far, which turns a linear pass into a quadratic one.

```codelaw
match:
  any:
    - pattern: "$XS.reduce(($ACC, $$$P) => [...$ACC, $$$R], $$$I)"
    - pattern: "$XS.reduce(($ACC, $$$P) => ({ ...$ACC, $$$R }), $$$I)"
    - pattern: "$XS.reduce(function ($ACC, $$$P) { return [...$ACC, $$$R]; }, $$$I)"
    - pattern: "$XS.reduce(function ($ACC, $$$P) { return { ...$ACC, $$$R }; }, $$$I)"
message: "Spreading the accumulator on every step is quadratic; push onto it or assign to it instead"
```

## How to fix

1. Mutate the accumulator (`acc.push(x); return acc;` or `acc[k] = v; return acc;`), or build the result with a loop.

## Examples

### JavaScript

#### Don't

```js
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

#### Do

```js
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```

### JSX

#### Don't

```jsx
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

#### Do

```jsx
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```

### TypeScript

#### Don't

```ts
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

#### Do

```ts
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```

### TSX

#### Don't

```tsx
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

#### Do

```tsx
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```
