---
id: js/no-negated-condition
title: "No negated condition with an else"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "swapping the branches moves code; the author confirms"
stability: preview
ported_from: [eslint/no-negated-condition, eslint-plugin-unicorn/no-negated-condition]
since: 0.5.0
---

An `if` with an `else`, or a conditional expression, does not test a negated condition.

**Why:** `if (!a) x else y` reads as a double negative; `if (a) y else x` says the same directly.

```codelaw
utils:
  negated:
    any:
    - {kind: unary_expression, regex: ^!}
    - kind: binary_expression
      has: {field: operator, regex: '^!==?$'}
match:
  any:
  - kind: if_statement
    all:
    - has:
        field: condition
        has: {matches: negated}
    - has:
        field: alternative
        not:
          has: {kind: if_statement}
  - kind: ternary_expression
    has: {field: condition, matches: negated}
message: Unexpected negated condition; swap the branches
```

## How to fix

1. Remove the negation and swap the two branches.

## Examples

### JavaScript

#### Don't

```js
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Do

```js
if (ok) {
  pass();
} else {
  fail();
}
```

### JSX

#### Don't

```jsx
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Do

```jsx
if (ok) {
  pass();
} else {
  fail();
}
```

### TypeScript

#### Don't

```ts
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Do

```ts
if (ok) {
  pass();
} else {
  fail();
}
```

### TSX

#### Don't

```tsx
if (!ok) {
  fail();
} else {
  pass();
}
```

#### Do

```tsx
if (ok) {
  pass();
} else {
  fail();
}
```
