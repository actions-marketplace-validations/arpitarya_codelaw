---
id: js/no-accidental-bitwise-operator
title: "No & or | in a condition"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`&&` short-circuits and yields an operand, while the bitwise test stays as is with an explicit nonzero check; the author picks"
stability: preview
ported_from: [eslint-plugin-unicorn/no-accidental-bitwise-operator]
since: 0.5.0
---

The whole condition of an `if`, `while` or `do … while` is not a bitwise `&` or `|`.

**Why:** In a condition `&` and `|` are almost always a typo for `&&` and `||`; they evaluate both sides and give a number.

```codelaw
match:
  kind: binary_expression
  all:
    - has: { field: operator, regex: '^(&|\|)$' }
    - inside:
        kind: parenthesized_expression
        inside: { any: [{kind: if_statement}, {kind: while_statement}, {kind: do_statement}], field: condition }
    - any:
        - { pattern: $A & $B, all: [{ pattern: $AND }] }
        - { pattern: $A | $B, all: [{ pattern: $OR }] }
message: "`&` and `|` are bitwise; use `&&` or `||` in a condition"
fix:
  choices:
    - label: logical operator
      default: true
      fix:
        - replace: { target: $AND, with: "$A && $B" }
        - replace: { target: $OR, with: "$A || $B" }
    - label: test the bits against zero
      fix:
        - replace: { target: $AND, with: "($A & $B) !== 0" }
        - replace: { target: $OR, with: "($A | $B) !== 0" }
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

#### Fixed (logical operator)

```js
if (ready && visible) {
  show();
}
```

#### Fixed (test the bits against zero)

```js
if ((ready & visible) !== 0) {
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

#### Fixed (logical operator)

```jsx
if (ready && visible) {
  show();
}
```

#### Fixed (test the bits against zero)

```jsx
if ((ready & visible) !== 0) {
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

#### Fixed (logical operator)

```ts
if (ready && visible) {
  show();
}
```

#### Fixed (test the bits against zero)

```ts
if ((ready & visible) !== 0) {
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

#### Fixed (logical operator)

```tsx
if (ready && visible) {
  show();
}
```

#### Fixed (test the bits against zero)

```tsx
if ((ready & visible) !== 0) {
  show();
}
```

#### Do

```tsx
if (ready && visible) {
  show();
}
```
