---
id: js/prefer-math-trunc
title: "Math.trunc rather than | 0"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`| 0` wraps values beyond 32 bits, `Math.trunc` does not"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-math-trunc]
since: 0.5.0
---

`x | 0` and `~~x` are written `Math.trunc(x)`.

**Why:** The bitwise trick only works for values in 32-bit range and hides what it is for.

```codelaw
match:
  pattern: "$X | 0"
message: "Use `Math.trunc()`"
fix: "Math.trunc($X)"
```

## Examples

### JavaScript

#### Don't

```js
const whole = ratio | 0;
```

#### Do

```js
const whole = Math.trunc(ratio);
```

### JSX

#### Don't

```jsx
const whole = ratio | 0;
```

#### Do

```jsx
const whole = Math.trunc(ratio);
```

### TypeScript

#### Don't

```ts
const whole = ratio | 0;
```

#### Do

```ts
const whole = Math.trunc(ratio);
```

### TSX

#### Don't

```tsx
const whole = ratio | 0;
```

#### Do

```tsx
const whole = Math.trunc(ratio);
```
