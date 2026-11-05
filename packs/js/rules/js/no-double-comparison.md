---
id: js/no-double-comparison
title: "x > a || x === a is x >= a"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the operator of the rewrite depends on which of four patterns matched"
stability: preview
ported_from: [eslint-plugin-unicorn/no-double-comparison]
since: 0.5.0
---

A strict comparison and an equality test of the same two operands, joined by `||`, are written as one `>=` or `<=`.

**Why:** One operator says the same with half the reading.

```codelaw

match:
  kind: binary_expression
  any:
    - pattern: $X > $Y || $X === $Y
    - pattern: $X < $Y || $X === $Y
    - pattern: $X === $Y || $X > $Y
    - pattern: $X === $Y || $X < $Y
message: "These two comparisons are one: use `>=` or `<=`"
```

## How to fix

1. Write `x >= y` or `x <= y`.

## Examples

### JavaScript

#### Don't

```js
if (n > limit || n === limit) {
  stop();
}
```

#### Do

```js
if (n >= limit) {
  stop();
}
```

### JSX

#### Don't

```jsx
if (n > limit || n === limit) {
  stop();
}
```

#### Do

```jsx
if (n >= limit) {
  stop();
}
```

### TypeScript

#### Don't

```ts
if (n > limit || n === limit) {
  stop();
}
```

#### Do

```ts
if (n >= limit) {
  stop();
}
```

### TSX

#### Don't

```tsx
if (n > limit || n === limit) {
  stop();
}
```

#### Do

```tsx
if (n >= limit) {
  stop();
}
```
