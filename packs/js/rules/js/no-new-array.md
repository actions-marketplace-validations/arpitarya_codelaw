---
id: js/no-new-array
title: "No new Array(n)"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether a length or a single element was meant decides the rewrite"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-unicorn/no-new-array]
since: 0.5.0
---

`new Array` is not called with one argument.

**Why:** `new Array(3)` makes three empty slots but `new Array("3")` an array holding "3": one call, two meanings.

```codelaw
match:
  pattern: "new Array($N)"
message: "Use `Array.from({ length })` or an array literal"
```

## How to fix

1. For a length, write `Array.from({ length: n })`; for one element, `[x]`.

## Examples

### JavaScript

#### Don't

```js
const slots = new Array(size);
```

#### Do

```js
const slots = Array.from({ length: size });
```

### JSX

#### Don't

```jsx
const slots = new Array(size);
```

#### Do

```jsx
const slots = Array.from({ length: size });
```

### TypeScript

#### Don't

```ts
const slots = new Array(size);
```

#### Do

```ts
const slots = Array.from({ length: size });
```

### TSX

#### Don't

```tsx
const slots = new Array(size);
```

#### Do

```tsx
const slots = Array.from({ length: size });
```
