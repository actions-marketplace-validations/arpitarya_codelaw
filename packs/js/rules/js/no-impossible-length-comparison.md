---
id: js/no-impossible-length-comparison
title: "No length < 0"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which comparison was meant (`=== 0`, `< 1`) is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-impossible-length-comparison]
since: 0.5.0
---

A `.length` or `.size` is not compared as less than 0.

**Why:** It is never negative, so the branch is dead; usually `=== 0` was meant.

```codelaw

match:
  kind: binary_expression
  any:
    - pattern: $X.length < 0
    - pattern: $X.size < 0
    - pattern: 0 > $X.length
    - pattern: 0 > $X.size
message: "A length is never negative; this comparison is always false"
```

## How to fix

1. Compare with what was meant, usually `=== 0`.

## Examples

### JavaScript

#### Don't

```js
if (items.length < 0) {
  fail();
}
```

#### Do

```js
if (items.length === 0) {
  fail();
}
```

### JSX

#### Don't

```jsx
if (items.length < 0) {
  fail();
}
```

#### Do

```jsx
if (items.length === 0) {
  fail();
}
```

### TypeScript

#### Don't

```ts
if (items.length < 0) {
  fail();
}
```

#### Do

```ts
if (items.length === 0) {
  fail();
}
```

### TSX

#### Don't

```tsx
if (items.length < 0) {
  fail();
}
```

#### Do

```tsx
if (items.length === 0) {
  fail();
}
```
