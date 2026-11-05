---
id: js/prefer-modern-math-apis
title: "Math.log10 rather than Math.log(x) * Math.LOG10E"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-modern-math-apis]
since: 0.5.0
---

Hand-written logarithms in base 10 or 2 use `Math.log10`/`Math.log2`. This port covers `Math.log(x) * Math.LOG10E`.

**Why:** The modern function is exact and readable.

```codelaw
match:
  pattern: "Math.log($X) * Math.LOG10E"
message: "Use `Math.log10()`"
fix: "Math.log10($X)"
```

## Examples

### JavaScript

#### Don't

```js
const d = Math.log(n) * Math.LOG10E;
```

#### Do

```js
const d = Math.log10(n);
```

### JSX

#### Don't

```jsx
const d = Math.log(n) * Math.LOG10E;
```

#### Do

```jsx
const d = Math.log10(n);
```

### TypeScript

#### Don't

```ts
const d = Math.log(n) * Math.LOG10E;
```

#### Do

```ts
const d = Math.log10(n);
```

### TSX

#### Don't

```tsx
const d = Math.log(n) * Math.LOG10E;
```

#### Do

```tsx
const d = Math.log10(n);
```
