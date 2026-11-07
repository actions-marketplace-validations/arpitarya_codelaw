---
id: js/no-unneeded-ternary
title: "No x ? true : false"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a condition that is not already a boolean becomes `Boolean(…)`, which the author may prefer to write another way"
stability: preview
ported_from: [eslint/no-unneeded-ternary]
since: 0.5.0
---

A ternary does not choose between `true` and `false`.

**Why:** The condition already is the boolean.

```codelaw

match:
  kind: ternary_expression
  any:
    - pattern: "$C ? true : false"
    - pattern: "$C ? false : true"
  all:
    - any:
      - all: [{ pattern: "$C ? true : false" }, { has: { field: condition, any: [{ kind: unary_expression, regex: '^!' }, { kind: binary_expression, has: { field: operator, regex: '^(===?|!==?|<=?|>=?|instanceof|in)$' } }] } }, { pattern: $SAME }]
      - all: [{ pattern: "$C ? true : false" }, { pattern: $WRAP }]
      - all: [{ pattern: "$C ? false : true" }, { has: { field: condition, any: [{ kind: identifier }, { kind: member_expression }, { kind: call_expression }, { kind: parenthesized_expression }] } }, { pattern: $NOT }]
      - all: [{ pattern: "$C ? false : true" }, { pattern: $NOTP }]
message: "Use the condition itself (or `!`) rather than a ternary of booleans"
fix:
  - replace: { target: $SAME, with: "$C" }
  - replace: { target: $WRAP, with: "Boolean($C)" }
  - replace: { target: $NOT, with: "!$C" }
  - replace: { target: $NOTP, with: "!($C)" }
```

## How to fix

1. Replace `c ? true : false` with `c` (or `Boolean(c)` if it is not a boolean), and `c ? false : true` with `!c`.

## Examples

### JavaScript

#### Don't

```js
const isAdult = age >= 18 ? true : false;
```

#### Do

```js
const isAdult = age >= 18;
```

### JSX

#### Don't

```jsx
const isAdult = age >= 18 ? true : false;
```

#### Do

```jsx
const isAdult = age >= 18;
```

### TypeScript

#### Don't

```ts
const isAdult = age >= 18 ? true : false;
```

#### Do

```ts
const isAdult = age >= 18;
```

### TSX

#### Don't

```tsx
const isAdult = age >= 18 ? true : false;
```

#### Do

```tsx
const isAdult = age >= 18;
```
