---
id: js/no-sequences
title: "No comma operator"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the expressions become separate statements, which changes the code shape"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/no-sequences]
since: 0.5.0
---

The comma operator is used only in a `for` loop's header or inside its own parentheses.

**Why:** `a, b` evaluates and discards `a`: easy to write by mistake and hard to spot.

```codelaw
match:
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
message: Unexpected comma operator; split it into statements
```

## How to fix

1. Write each expression as its own statement.

## Examples

### JavaScript

#### Don't

```js
a = 1, b = 2;
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

#### Do

```tsx
a = 1;
b = 2;
```
