---
id: js/no-else-return
title: "No else after return"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "unwrapping the block re-indents its statements, and a declaration in it could clash with one outside"
fix_pending: "suggest; needs computed-edit"
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
      has:
        kind: statement_block
        pattern: $B
message: The `if` returns, so the `else` is not needed
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
