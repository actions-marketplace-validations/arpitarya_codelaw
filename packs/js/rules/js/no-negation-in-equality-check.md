---
id: js/no-negation-in-equality-check
title: "No !a === b"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether `a !== b` or `!(a === b)` was meant is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-negation-in-equality-check]
since: 0.5.0
---

The left operand of an equality check is not a negation (`!a === b`).

**Why:** `!` binds tighter than `===`, so the code compares a boolean with `b`.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^(===|!==|==|!=)$
  all:
  - has:
      field: left
      kind: unary_expression
      regex: ^!
message: '`!a === b` negates `a` first; you probably meant `a !== b`'
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
if (!type === "admin") {
  deny();
}
```

#### Do

```js
if (type !== "admin") {
  deny();
}
```

### JSX

#### Don't

```jsx
if (!type === "admin") {
  deny();
}
```

#### Do

```jsx
if (type !== "admin") {
  deny();
}
```

### TypeScript

#### Don't

```ts
if (!type === "admin") {
  deny();
}
```

#### Do

```ts
if (type !== "admin") {
  deny();
}
```

### TSX

#### Don't

```tsx
if (!type === "admin") {
  deny();
}
```

#### Do

```tsx
if (type !== "admin") {
  deny();
}
```
