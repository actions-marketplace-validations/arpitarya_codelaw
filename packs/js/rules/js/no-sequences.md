---
id: js/no-sequences
title: "No comma operator"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a comma sequence that is a whole expression statement or a return value splits into statements with the same evaluation order (two expressions); longer sequences and sequences elsewhere carry no edit"
stability: preview
ported_from: [eslint/no-sequences]
since: 0.5.0
---

The comma operator is used only in a `for` loop's header or inside its own parentheses.

**Why:** `a, b` evaluates and discards `a`: easy to write by mistake and hard to spot.

```codelaw
utils:
  plain:
    kind: sequence_expression
    all:
    - not:
        inside: {kind: sequence_expression}
    - not:
        inside: {kind: parenthesized_expression}
    - not:
        inside: {kind: for_statement}
    - not:
        inside:
          kind: expression_statement
          inside: {kind: for_statement}
match:
  any:
  - matches: plain
    pattern: $M1
    not: { has: { nthChild: 3, pattern: $X3 } }
    inside: { kind: expression_statement, stopBy: neighbor }
    all:
    - has: { nthChild: 1, pattern: $L1 }
    - has: { nthChild: 2, pattern: $R1 }
  - matches: plain
    inside: { kind: return_statement, stopBy: neighbor, pattern: $RS }
    not: { has: { nthChild: 3, pattern: $X4 } }
    all:
    - has: { nthChild: 1, pattern: $L2 }
    - has: { nthChild: 2, pattern: $R2 }
  - matches: plain
message: Unexpected comma operator; split it into statements
fix:
  - replace: { target: $M1, with: "$L1; $R1" }
  - replace: { target: $RS, with: "$L2; return $R2;" }
```

## How to fix

1. Write each expression as its own statement.

## Examples

### JavaScript

#### Don't

```js
a = 1, b = 2;
```

#### Fixed

```js
a = 1; b = 2;
```

#### Do

```js
a = 1;
b = 2;
```

### JSX

#### Don't

```jsx
a = 1, b = 2;
```

#### Fixed

```jsx
a = 1; b = 2;
```

#### Do

```jsx
a = 1;
b = 2;
```

### TypeScript

#### Don't

```ts
a = 1, b = 2;
```

#### Fixed

```ts
a = 1; b = 2;
```

#### Do

```ts
a = 1;
b = 2;
```

### TSX

#### Don't

```tsx
a = 1, b = 2;
```

#### Fixed

```tsx
a = 1; b = 2;
```

#### Do

```tsx
a = 1;
b = 2;
```
