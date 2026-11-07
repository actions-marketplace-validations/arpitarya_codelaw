---
id: js/no-constant-binary-expression
title: "No binary expression whose result is fixed"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the choices are dropping the operand that never decides the result (the default, behaviour-preserving) or regrouping, `a + b ?? c` as `a + (b ?? c)`; a constant comparison is manual"
fix_exception: value
stability: preview
ported_from: [eslint/no-constant-binary-expression]
since: 0.1.0
---

The left side of `??`, `||` and `&&` is not a value whose nullishness or truthiness is fixed, and
nothing is compared with a newly created object or with `null` when it can never be `null`.

**Why:** Such an expression always gives the same answer, which usually means the author misread
operator precedence.

```codelaw
native: js/no-constant-binary-expression
```

## How to fix

1. Add the parentheses the expression was meant to have.
2. Or compare with the value that was meant.

## Examples

### JavaScript

#### Don't

```js
const total = price + tax ?? 0;
```

#### Fixed

```js
const total = price + tax;
```

#### Do

```js
const total = price + (tax ?? 0);
```

### JSX

#### Don't

```jsx
const total = price + tax ?? 0;
```

#### Fixed

```jsx
const total = price + tax;
```

#### Do

```jsx
const total = price + (tax ?? 0);
```

### TypeScript

#### Don't

```ts
const total = price + tax ?? 0;
```

#### Fixed

```ts
const total = price + tax;
```

#### Do

```ts
const total = price + (tax ?? 0);
```

### TSX

#### Don't

```tsx
const total = price + tax ?? 0;
```

#### Fixed

```tsx
const total = price + tax;
```

#### Do

```tsx
const total = price + (tax ?? 0);
```
