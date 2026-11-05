---
id: js/no-duplicate-logical-operands
title: "No a && a"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-duplicate-logical-operands]
since: 0.5.0
---

The two operands of `&&`, `||` or `??` are not the same expression. Operands that call a function are left alone, since a call can return something different each time.

**Why:** The repeat is redundant at best and usually a typo for a different operand.

```codelaw

match:
  kind: binary_expression
  any: [{ pattern: "$A && $A" }, { pattern: "$A || $A" }, { pattern: "$A ?? $A" }]
  not: { has: { kind: call_expression, stopBy: end } }
message: "Both operands are `$A`; one is enough"
fix: "$A"
```

## Examples

### JavaScript

#### Don't

```js
if (ready && ready) {
  go();
}
```

#### Do

```js
if (ready) {
  go();
}
```

### JSX

#### Don't

```jsx
if (ready && ready) {
  go();
}
```

#### Do

```jsx
if (ready) {
  go();
}
```

### TypeScript

#### Don't

```ts
if (ready && ready) {
  go();
}
```

#### Do

```ts
if (ready) {
  go();
}
```

### TSX

#### Don't

```tsx
if (ready && ready) {
  go();
}
```

#### Do

```tsx
if (ready) {
  go();
}
```
