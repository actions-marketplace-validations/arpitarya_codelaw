---
id: js/prefer-math-constants
title: "Math.SQRT2 rather than Math.sqrt(2)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-math-constants]
since: 0.5.0
---

`Math.sqrt(2)` and `Math.sqrt(0.5)` are written `Math.SQRT2` and `Math.SQRT1_2`. This port covers `Math.sqrt(2)`.

**Why:** The constant is computed once and exactly.

```codelaw
match:
  pattern: "Math.sqrt(2)"
message: "Use `Math.SQRT2`"
fix: "Math.SQRT2"
```

## Examples

### JavaScript

#### Don't

```js
const d = side * Math.sqrt(2);
```

#### Do

```js
const d = side * Math.SQRT2;
```

### JSX

#### Don't

```jsx
const d = side * Math.sqrt(2);
```

#### Do

```jsx
const d = side * Math.SQRT2;
```

### TypeScript

#### Don't

```ts
const d = side * Math.sqrt(2);
```

#### Do

```ts
const d = side * Math.SQRT2;
```

### TSX

#### Don't

```tsx
const d = side * Math.sqrt(2);
```

#### Do

```tsx
const d = side * Math.SQRT2;
```
