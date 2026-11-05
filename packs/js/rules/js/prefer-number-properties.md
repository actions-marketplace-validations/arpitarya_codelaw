---
id: js/prefer-number-properties
title: "Number.isNaN rather than isNaN"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the `Number` method does not convert its argument, so a string gives a different answer"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-number-properties]
since: 0.5.0
---

The global `isNaN` and `isFinite` are written as `Number` methods. Upstream also covers `parseInt`, `parseFloat` and the constants; this port covers these two.

**Why:** The globals convert their argument first, so `isNaN("abc")` is true.

```codelaw

match:
  pattern: $F($X)
constraints:
  F: { regex: '^(isNaN|isFinite)$' }
where:
  $F: builtin
message: "Use `Number.$F()`; the global converts its argument first"
fix: "Number.$F($X)"
```

## Examples

### JavaScript

#### Don't

```js
if (isNaN(value)) {
  reset();
}
```

#### Do

```js
if (Number.isNaN(value)) {
  reset();
}
```

### JSX

#### Don't

```jsx
if (isNaN(value)) {
  reset();
}
```

#### Do

```jsx
if (Number.isNaN(value)) {
  reset();
}
```

### TypeScript

#### Don't

```ts
if (isNaN(value)) {
  reset();
}
```

#### Do

```ts
if (Number.isNaN(value)) {
  reset();
}
```

### TSX

#### Don't

```tsx
if (isNaN(value)) {
  reset();
}
```

#### Do

```tsx
if (Number.isNaN(value)) {
  reset();
}
```
