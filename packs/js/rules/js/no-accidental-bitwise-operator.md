---
id: js/no-accidental-bitwise-operator
title: "No & or | in a condition"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`&&` short-circuits and yields an operand rather than a number"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-unicorn/no-accidental-bitwise-operator]
since: 0.5.0
---

The whole condition of an `if`, `while` or `do … while` is not a bitwise `&` or `|`.

**Why:** In a condition `&` and `|` are almost always a typo for `&&` and `||`; they evaluate both sides and give a number.

```codelaw

match:
  kind: binary_expression
  has: { field: operator, regex: '^(&|\|)$' }
  inside:
    kind: parenthesized_expression
    inside: { any: [{kind: if_statement}, {kind: while_statement}, {kind: do_statement}], field: condition }
message: "`&` and `|` are bitwise; use `&&` or `||` in a condition"
```

## How to fix

1. Use `&&` for `&` and `||` for `|`; if a bit test was meant, compare it: `(flags & MASK) !== 0`.

## Examples

### JavaScript

#### Don't

```js
if (ready & visible) {
  show();
}
```

#### Do

```js
if (ready && visible) {
  show();
}
```

### JSX

#### Don't

```jsx
if (ready & visible) {
  show();
}
```

#### Do

```jsx
if (ready && visible) {
  show();
}
```

### TypeScript

#### Don't

```ts
if (ready & visible) {
  show();
}
```

#### Do

```ts
if (ready && visible) {
  show();
}
```

### TSX

#### Don't

```tsx
if (ready & visible) {
  show();
}
```

#### Do

```tsx
if (ready && visible) {
  show();
}
```
