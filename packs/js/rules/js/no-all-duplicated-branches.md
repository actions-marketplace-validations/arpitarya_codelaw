---
id: js/no-all-duplicated-branches
title: "Not every branch the same"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which branch is wrong is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-all-duplicated-branches]
since: 0.5.0
---

A ternary or an `if … else` does not have the same code in both branches.

**Why:** The condition is evaluated for nothing, which usually means one branch was copied and not changed.

```codelaw
match:
  any:
  - kind: ternary_expression
    pattern: '$C ? $X : $X'
  - kind: if_statement
    has:
      field: consequence
      pattern: $B
    all:
    - has:
        field: alternative
        kind: else_clause
        has:
          pattern: $B
          kind: statement_block
message: Both branches do the same thing; the condition has no effect
```

## How to fix

1. Change the branch that was meant to differ, or drop the condition.

## Examples

### JavaScript

#### Don't

```js
const fee = vip ? 0 : 0;
```

#### Do

```js
const fee = vip ? 0 : 5;
```

### JSX

#### Don't

```jsx
const fee = vip ? 0 : 0;
```

#### Do

```jsx
const fee = vip ? 0 : 5;
```

### TypeScript

#### Don't

```ts
const fee = vip ? 0 : 0;
```

#### Do

```ts
const fee = vip ? 0 : 5;
```

### TSX

#### Don't

```tsx
const fee = vip ? 0 : 0;
```

#### Do

```tsx
const fee = vip ? 0 : 5;
```
