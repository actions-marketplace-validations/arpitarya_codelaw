---
id: js/no-unneeded-ternary
title: "No x ? true : false"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a non-boolean condition needs `Boolean(…)` or `!`, decided per pattern"
stability: preview
ported_from: [eslint/no-unneeded-ternary]
since: 0.5.0
---

A ternary does not choose between `true` and `false`.

**Why:** The condition already is the boolean.

```codelaw

match:
  kind: ternary_expression
  any:
    - pattern: "$C ? true : false"
    - pattern: "$C ? false : true"
message: "Use the condition itself (or `!`) rather than a ternary of booleans"
```

## How to fix

1. Replace `c ? true : false` with `c` (or `Boolean(c)` if it is not a boolean), and `c ? false : true` with `!c`.

## Examples

### JavaScript

#### Don't

```js
const isAdult = age >= 18 ? true : false;
```

#### Do

```js
const isAdult = age >= 18;
```

### JSX

#### Don't

```jsx
const isAdult = age >= 18 ? true : false;
```

#### Do

```jsx
const isAdult = age >= 18;
```

### TypeScript

#### Don't

```ts
const isAdult = age >= 18 ? true : false;
```

#### Do

```ts
const isAdult = age >= 18;
```

### TSX

#### Don't

```tsx
const isAdult = age >= 18 ? true : false;
```

#### Do

```tsx
const isAdult = age >= 18;
```
