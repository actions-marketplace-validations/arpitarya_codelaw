---
id: js/no-self-compare
title: "No comparing a value with itself"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the intended comparison is the author's"
stability: preview
ported_from: [eslint/no-self-compare]
since: 0.5.0
---

An expression is not compared with itself.

**Why:** `x === x` is true except for `NaN` — use `Number.isNaN` for that — and is otherwise a typo.

```codelaw
match:
  any:
    - pattern: $A === $A
    - pattern: $A == $A
    - pattern: $A !== $A
    - pattern: $A != $A
    - pattern: $A < $A
    - pattern: $A > $A
    - pattern: $A <= $A
    - pattern: $A >= $A
message: "Comparing `$A` with itself"
```

## How to fix

1. Compare with the value meant, or use `Number.isNaN(x)` to test for `NaN`.

## Examples

### JavaScript

#### Don't

```js
if (x !== x) skip();
```

#### Do

```js
if (Number.isNaN(x)) skip();
```

### JSX

#### Don't

```jsx
if (x !== x) skip();
```

#### Do

```jsx
if (Number.isNaN(x)) skip();
```

### TypeScript

#### Don't

```ts
if (x !== x) skip();
```

#### Do

```ts
if (Number.isNaN(x)) skip();
```

### TSX

#### Don't

```tsx
if (x !== x) skip();
```

#### Do

```tsx
if (Number.isNaN(x)) skip();
```
