---
id: js/prefer-array-from-range
title: "Array.from({ length }) rather than spreading Array(n).keys()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-from-range]
since: 0.5.0
---

The indexes `0 … n - 1` as an array are `Array.from({ length: n }, (_, index) => index)`, not `[...Array(n).keys()]` or `Array.from(Array(n).keys())` (with or without `new`).

**Why:** `Array.from` with a length builds the array in one step, without a sparse array and an iterator in between.

```codelaw
match:
  any:
    - pattern: "[...Array($N).keys()]"
    - pattern: "[...new Array($N).keys()]"
    - pattern: "Array.from(Array($N).keys())"
    - pattern: "Array.from(new Array($N).keys())"
constraints:
  N: { not: { kind: spread_element } }
message: "Build the range with `Array.from({ length })`"
fix: "Array.from({ length: $N }, (_, index) => index)"
```

## Examples

### JavaScript

#### Don't

```js
const indexes = [...Array(count).keys()];
```

#### Do

```js
const indexes = Array.from({ length: count }, (_, index) => index);
```


### JSX

#### Don't

```jsx
const indexes = [...Array(count).keys()];
```

#### Do

```jsx
const indexes = Array.from({ length: count }, (_, index) => index);
```


### TypeScript

#### Don't

```ts
const indexes = [...Array(count).keys()];
```

#### Do

```ts
const indexes = Array.from({ length: count }, (_, index) => index);
```


### TSX

#### Don't

```tsx
const indexes = [...Array(count).keys()];
```

#### Do

```tsx
const indexes = Array.from({ length: count }, (_, index) => index);
```
