---
id: js/use-simplified-logic-expression
title: "Simplify a logical expression"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [biome/complexity/useSimplifiedLogicExpression]
since: 0.5.1
---

Three forms are rewritten: an operand that cannot change the result (`true && x`, `false || x`, `null ?? x`, `undefined ?? x`) is dropped, and two negations joined by `||` or `&&` become one negation (`!a || !b` is `!(a && b)`, `!a && !b` is `!(a || b)`; De Morgan's law). `x || false` is left alone: outside a boolean test it is not `x`. Behaviour of Biome's `useSimplifiedLogicExpression`, the forms that need no knowledge of the operands' types.

**Why:** A literal operand is dead code a reader has to evaluate, and one negation over a group reads as one condition instead of two.

```codelaw
match:
  any:
    - all: [{ pattern: $A }, { pattern: "true && $X" }]
    - all: [{ pattern: $B }, { pattern: "false || $X" }]
    - all: [{ pattern: $C }, { pattern: "null ?? $X" }]
    - all: [{ pattern: $D }, { pattern: "undefined ?? $X" }]
    - all: [{ pattern: $E }, { pattern: "!$X || !$Y" }]
    - all: [{ pattern: $F }, { pattern: "!$X && !$Y" }]
message: "This logical expression can be written more simply"
fix:
  - replace: { target: $A, with: "$X" }
  - replace: { target: $B, with: "$X" }
  - replace: { target: $C, with: "$X" }
  - replace: { target: $D, with: "$X" }
  - replace: { target: $E, with: "!($X && $Y)" }
  - replace: { target: $F, with: "!($X || $Y)" }
```

## Examples

### JavaScript

#### Don't

```js
const ok = !a || !b;
```

#### Do

```js
const ok = !(a && b);
```

#### Fixed

```js
const ok = !(a && b);
```

### JSX

#### Don't

```jsx
const ok = !a || !b;
```

#### Do

```jsx
const ok = !(a && b);
```

#### Fixed

```jsx
const ok = !(a && b);
```

### TypeScript

#### Don't

```ts
const ok = !a || !b;
```

#### Do

```ts
const ok = !(a && b);
```

#### Fixed

```ts
const ok = !(a && b);
```

### TSX

#### Don't

```tsx
const ok = !a || !b;
```

#### Do

```tsx
const ok = !(a && b);
```

#### Fixed

```tsx
const ok = !(a && b);
```
