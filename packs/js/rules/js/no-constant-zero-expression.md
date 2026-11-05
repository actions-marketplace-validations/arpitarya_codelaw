---
id: js/no-constant-zero-expression
title: "No multiply by zero"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether 0 or another factor was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-unicorn/no-constant-zero-expression]
since: 0.5.0
---

A value is not multiplied by the literal `0`.

**Why:** The result is always 0 (or `NaN`), so the expression is a leftover or a typo.

```codelaw

match:
  kind: binary_expression
  any: [{ pattern: $X * 0 }, { pattern: 0 * $X }]
message: "Multiplying by 0 always gives 0"
```

## How to fix

1. Write the constant, or the factor that was meant.

## Examples

### JavaScript

#### Don't

```js
const offset = width * 0;
```

#### Do

```js
const offset = 0;
```

### JSX

#### Don't

```jsx
const offset = width * 0;
```

#### Do

```jsx
const offset = 0;
```

### TypeScript

#### Don't

```ts
const offset = width * 0;
```

#### Do

```ts
const offset = 0;
```

### TSX

#### Don't

```tsx
const offset = width * 0;
```

#### Do

```tsx
const offset = 0;
```
