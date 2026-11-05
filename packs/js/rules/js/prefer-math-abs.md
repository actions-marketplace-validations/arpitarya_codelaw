---
id: js/prefer-math-abs
title: "Math.abs rather than a ternary"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Math.abs` converts its argument to a number, the ternary does not"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-math-abs]
since: 0.5.0
---

A ternary that negates negative values is written `Math.abs`.

**Why:** The function names the intent.

```codelaw
match:
  pattern: "$X < 0 ? -$X : $X"
message: "Use `Math.abs()`"
fix: "Math.abs($X)"
```

## Examples

### JavaScript

#### Don't

```js
const d = n < 0 ? -n : n;
```

#### Do

```js
const d = Math.abs(n);
```

### JSX

#### Don't

```jsx
const d = n < 0 ? -n : n;
```

#### Do

```jsx
const d = Math.abs(n);
```

### TypeScript

#### Don't

```ts
const d = n < 0 ? -n : n;
```

#### Do

```ts
const d = Math.abs(n);
```

### TSX

#### Don't

```tsx
const d = n < 0 ? -n : n;
```

#### Do

```tsx
const d = Math.abs(n);
```
