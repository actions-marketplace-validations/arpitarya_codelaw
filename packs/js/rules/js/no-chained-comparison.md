---
id: js/no-chained-comparison
title: "No a < b < c"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the middle operand is evaluated twice in the rewrite, and the comparison the author meant may not be the range; where `&&` would bind differently with what surrounds it, the rewrite is by hand"
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
    pattern: $$O2
  all:
  - has:
      field: left
      kind: binary_expression
      has:
        field: operator
        regex: ^(<|>|<=|>=)$
        pattern: $$O1
      all:
      - has: { field: left, pattern: $A }
      - has:
          field: right
          pattern: $B
  - has: { field: right, pattern: $C }
  - any:
      - all:
          - inside: { any: [{ kind: parenthesized_expression }, { kind: variable_declarator }, { kind: return_statement }, { kind: arguments }, { kind: expression_statement }] }
          - pattern: $CMP
      - regex: ''
message: '`a < b < c` compares a boolean with `c`; write `a < b && b < c`'
fix:
  - replace: { target: $CMP, with: "$A $O1 $B && $B $O2 $C" }
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
