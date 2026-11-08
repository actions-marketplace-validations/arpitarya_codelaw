---
id: js/no-array-from-fill
title: "Array.from with a mapper rather than fill()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`fill` evaluates its value once and the mapper once per element: a call or an object literal there gives each element its own value; a `fill` with a start or end, or an `Array.from` with a mapper already, is rewritten by hand"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-from-fill]
since: 0.5.0
---

`Array.from(…).fill(…)` is written as `Array.from(…, () => value)`.

**Why:** It makes one array instead of two passes, and the mapper creates a fresh value each time.

```codelaw

match:
  kind: call_expression
  pattern: Array.from($$$A).fill($$$B)
  any:
    - all: [{ pattern: Array.from($ONE).fill($VAL) }, { pattern: $CALL }]
    - regex: ''
message: "`Array.from({ length }, () => value)` builds the array in one step"
fix:
  - replace: { target: $CALL, with: "Array.from($ONE, () => $VAL|arrow-body)" }
```

## How to fix

1. Move the fill value into a mapper: `Array.from({ length: n }, () => value)`.

## Examples

### JavaScript

#### Don't

```js
const zeros = Array.from({ length: n }).fill(0);
```

#### Do

```js
const zeros = Array.from({ length: n }, () => 0);
```

### JSX

#### Don't

```jsx
const zeros = Array.from({ length: n }).fill(0);
```

#### Do

```jsx
const zeros = Array.from({ length: n }, () => 0);
```

### TypeScript

#### Don't

```ts
const zeros = Array.from({ length: n }).fill(0);
```

#### Do

```ts
const zeros = Array.from({ length: n }, () => 0);
```

### TSX

#### Don't

```tsx
const zeros = Array.from({ length: n }).fill(0);
```

#### Do

```tsx
const zeros = Array.from({ length: n }, () => 0);
```
