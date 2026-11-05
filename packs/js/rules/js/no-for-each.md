---
id: js/no-for-each
title: "for…of rather than forEach"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a `return` in the callback becomes `continue`, and an index parameter becomes `.entries()`"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-unicorn/no-for-each]
since: 0.5.0
---

A statement calling `.forEach` with an arrow function is written as a `for … of` loop.

**Why:** A loop can `break`, `continue`, `return` and `await`; a `forEach` callback can do none of them.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has:
        field: property
        regex: ^forEach$
    all:
    - has:
        field: arguments
        has:
          nthChild: 1
          kind: arrow_function
message: Use a `for … of` loop instead of `.forEach`
```

## How to fix

1. Rewrite as `for (const item of items) { … }`; turn `return` into `continue`; for an index parameter use `for (const [i, item] of items.entries())`.

## Examples

### JavaScript

#### Don't

```js
items.forEach((item) => {
  send(item);
});
```

#### Do

```js
for (const item of items) {
  send(item);
}
```

### JSX

#### Don't

```jsx
items.forEach((item) => {
  send(item);
});
```

#### Do

```jsx
for (const item of items) {
  send(item);
}
```

### TypeScript

#### Don't

```ts
items.forEach((item) => {
  send(item);
});
```

#### Do

```ts
for (const item of items) {
  send(item);
}
```

### TSX

#### Don't

```tsx
items.forEach((item) => {
  send(item);
});
```

#### Do

```tsx
for (const item of items) {
  send(item);
}
```
