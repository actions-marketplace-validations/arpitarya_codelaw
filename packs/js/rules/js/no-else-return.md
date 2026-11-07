---
id: js/no-else-return
title: "No else after return"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a declaration in the `else` block moves into the enclosing scope, where it could clash with one of the same name"
stability: preview
ported_from: [eslint/no-else-return, eslint-plugin-unicorn/no-useless-else]
since: 0.5.0
---

An `else` block does not follow an `if` block that ends in `return`.

**Why:** The `else` only adds nesting; the code after the `if` runs exactly when the condition is false.

```codelaw
match:
  kind: if_statement
  has:
    field: consequence
    kind: statement_block
    has:
      kind: return_statement
  inside:
    kind: statement_block
  all:
  - has:
      field: alternative
      kind: else_clause
      pattern: $E
      has:
        kind: statement_block
        pattern: $B
message: The `if` returns, so the `else` is not needed
fix:
  - unwrap-block: { target: $B, within: $E }
```

## How to fix

1. Delete `else {` and its closing `}`, and move its statements up one level.

## Examples

### JavaScript

#### Don't

```js
function sign(n) {
  if (n < 0) {
    return -1;
  } else {
    return 1;
  }
}
```

#### Do

```js
function sign(n) {
  if (n < 0) {
    return -1;
  }
  return 1;
}
```

### JSX

#### Don't

```jsx
function sign(n) {
  if (n < 0) {
    return -1;
  } else {
    return 1;
  }
}
```

#### Do

```jsx
function sign(n) {
  if (n < 0) {
    return -1;
  }
  return 1;
}
```

### TypeScript

#### Don't

```ts
function sign(n) {
  if (n < 0) {
    return -1;
  } else {
    return 1;
  }
}
```

#### Do

```ts
function sign(n) {
  if (n < 0) {
    return -1;
  }
  return 1;
}
```

### TSX

#### Don't

```tsx
function sign(n) {
  if (n < 0) {
    return -1;
  } else {
    return 1;
  }
}
```

#### Do

```tsx
function sign(n) {
  if (n < 0) {
    return -1;
  }
  return 1;
}
```
