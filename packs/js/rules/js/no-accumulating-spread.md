---
id: js/no-accumulating-spread
title: "No spread of the accumulator in reduce"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a push onto an array-literal start value nothing else holds is auto; any other start value, a nested function in the elements, or the object form is a suggestion"
stability: preview
ported_from: [biome/performance/noAccumulatingSpread]
since: 0.5.0
---

A `reduce` callback does not build a new array or object by spreading its accumulator on every step.

**Why:** Each step copies everything accumulated so far, which turns a linear pass into a quadratic one.

```codelaw
match:
  any:
    - all: [{ pattern: $W1 }, { pattern: "$XS.reduce(($ACC, $$$P) => [...$ACC, $$$R], $$$I)" }]
    - all: [{ pattern: $W2 }, { pattern: "$XS.reduce(($ACC, $$$P) => ({ ...$ACC, $$$R }), $$$I)" }]
    - all: [{ pattern: $W3 }, { pattern: "$XS.reduce(function ($ACC, $$$P) { return [...$ACC, $$$R]; }, $$$I)" }]
    - all: [{ pattern: $W4 }, { pattern: "$XS.reduce(function ($ACC, $$$P) { return { ...$ACC, $$$R }; }, $$$I)" }]
auto_when:
  any:
    - all:
        - pattern: "$XS.reduce(($ACC, $$$P) => [...$ACC, $$$R], [$$$E])"
        - not:
            has:
              stopBy: end
              kind: array
              has: { stopBy: end, any: [{kind: arrow_function}, {kind: function_expression}, {kind: function}, {kind: class}] }
    - all:
        - pattern: "$XS.reduce(function ($ACC, $$$P) { return [...$ACC, $$$R]; }, [$$$E])"
        - not:
            has:
              stopBy: end
              kind: array
              has: { stopBy: end, any: [{kind: arrow_function}, {kind: function_expression}, {kind: function}, {kind: class}] }
message: "Spreading the accumulator on every step is quadratic; push onto it or assign to it instead"
fix:
  - replace: { target: $W1, with: "$XS.reduce(($ACC, $$$P) => {\n  $ACC.push($$$R);\n  return $ACC;\n}, $$$I)" }
  - replace: { target: $W2, with: "$XS.reduce(($ACC, $$$P) => Object.assign($ACC, { $$$R }), $$$I)" }
  - replace: { target: $W3, with: "$XS.reduce(function ($ACC, $$$P) {\n  $ACC.push($$$R);\n  return $ACC;\n}, $$$I)" }
  - replace: { target: $W4, with: "$XS.reduce(function ($ACC, $$$P) {\n  return Object.assign($ACC, { $$$R });\n}, $$$I)" }
```

## How to fix

1. Mutate the accumulator (`acc.push(x); return acc;` or `acc[k] = v; return acc;`), or build the result with a loop.

## Examples

### JavaScript

#### Don't

```js
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

#### Fixed

```js
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
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

#### Fixed

```jsx
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
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

#### Fixed

```ts
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
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

#### Fixed

```tsx
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```

#### Do

```tsx
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```
