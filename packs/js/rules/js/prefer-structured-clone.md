---
id: js/prefer-structured-clone
title: "structuredClone rather than JSON round-trip"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "dates stay `Date` objects, `undefined` keys are kept, and a function throws instead of being dropped"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-structured-clone]
since: 0.5.0
---

A deep copy is made with `structuredClone`, not `JSON.parse(JSON.stringify(x))`.

**Why:** `structuredClone` keeps dates, maps, sets and cycles, and is faster.

```codelaw
match:
  pattern: "JSON.parse(JSON.stringify($X))"
message: "Use `structuredClone()`"
fix: "structuredClone($X)"
```

## Examples

### JavaScript

#### Don't

```js
const copy = JSON.parse(JSON.stringify(state));
```

#### Do

```js
const copy = structuredClone(state);
```

### JSX

#### Don't

```jsx
const copy = JSON.parse(JSON.stringify(state));
```

#### Do

```jsx
const copy = structuredClone(state);
```

### TypeScript

#### Don't

```ts
const copy = JSON.parse(JSON.stringify(state));
```

#### Do

```ts
const copy = structuredClone(state);
```

### TSX

#### Don't

```tsx
const copy = JSON.parse(JSON.stringify(state));
```

#### Do

```tsx
const copy = structuredClone(state);
```
