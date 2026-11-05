---
id: js/no-constant-math-min-max-clamp
title: "No clamp with its bounds swapped"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "swapping the bounds makes the clamp work, which changes the value it returns"
stability: preview
ported_from: [biome/correctness/noConstantMathMinMaxClamp]
since: 0.5.0
---

`Math.min(lo, Math.max(hi, x))` with `lo` below `hi` (or the mirror form) always returns a bound.

**Why:** The clamp is constant: its bounds are the wrong way round.

```codelaw
native: js/no-constant-math-min-max-clamp
```

## How to fix

1. Swap the two bounds.

## Examples

### JavaScript

#### Don't

```js
const v = Math.min(0, Math.max(100, x));
```

#### Do

```js
const v = Math.min(100, Math.max(0, x));
```

### JSX

#### Don't

```jsx
const v = Math.min(0, Math.max(100, x));
```

#### Do

```jsx
const v = Math.min(100, Math.max(0, x));
```

### TypeScript

#### Don't

```ts
const v = Math.min(0, Math.max(100, x));
```

#### Do

```ts
const v = Math.min(100, Math.max(0, x));
```

### TSX

#### Don't

```tsx
const v = Math.min(0, Math.max(100, x));
```

#### Do

```tsx
const v = Math.min(100, Math.max(0, x));
```
