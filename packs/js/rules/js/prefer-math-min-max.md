---
id: js/prefer-math-min-max
title: "Math.min rather than a ternary"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Math.min` turns `NaN` and non-numbers into `NaN`, the ternary does not"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-math-min-max]
since: 0.5.0
---

A ternary that picks the smaller (or larger) of two values is written `Math.min`/`Math.max`. This port covers `a > b ? b : a`.

**Why:** The function names the choice.

```codelaw
match:
  pattern: "$A > $B ? $B : $A"
message: "Use `Math.min()`"
fix: "Math.min($A, $B)"
```

## Examples

### JavaScript

#### Don't

```js
const lo = a > b ? b : a;
```

#### Do

```js
const lo = Math.min(a, b);
```

### JSX

#### Don't

```jsx
const lo = a > b ? b : a;
```

#### Do

```jsx
const lo = Math.min(a, b);
```

### TypeScript

#### Don't

```ts
const lo = a > b ? b : a;
```

#### Do

```ts
const lo = Math.min(a, b);
```

### TSX

#### Don't

```tsx
const lo = a > b ? b : a;
```

#### Do

```tsx
const lo = Math.min(a, b);
```
