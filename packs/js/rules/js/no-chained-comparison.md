---
id: js/no-chained-comparison
title: "No a < b < c"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the middle operand is evaluated twice in the rewrite"
stability: preview
ported_from: [eslint-plugin-unicorn/no-chained-comparison]
since: 0.5.0
---

A comparison is not chained (`a < b < c`).

**Why:** JavaScript evaluates `a < b` to `true` or `false` first and compares that with `c`, so the range check is always wrong.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^(<|>|<=|>=)$
  all:
  - has:
      field: left
      kind: binary_expression
      has:
        field: operator
        regex: ^(<|>|<=|>=)$
      all:
      - has:
          field: right
          pattern: $B
message: '`a < b < c` compares a boolean with `c`; write `a < b && b < c`'
```

## How to fix

1. Split it into two comparisons joined by `&&`: `a < b && b < c`.

## Examples

### JavaScript

#### Don't

```js
if (0 <= x <= 10) {
  ok();
}
```

#### Do

```js
if (0 <= x && x <= 10) {
  ok();
}
```

### JSX

#### Don't

```jsx
if (0 <= x <= 10) {
  ok();
}
```

#### Do

```jsx
if (0 <= x && x <= 10) {
  ok();
}
```

### TypeScript

#### Don't

```ts
if (0 <= x <= 10) {
  ok();
}
```

#### Do

```ts
if (0 <= x && x <= 10) {
  ok();
}
```

### TSX

#### Don't

```tsx
if (0 <= x <= 10) {
  ok();
}
```

#### Do

```tsx
if (0 <= x && x <= 10) {
  ok();
}
```
