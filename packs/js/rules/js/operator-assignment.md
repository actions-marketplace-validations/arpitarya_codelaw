---
id: js/operator-assignment
title: "x += 1 rather than x = x + 1"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the operator token cannot be carried into a fix template"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint/operator-assignment, eslint-plugin-unicorn/operator-assignment]
since: 0.5.0
---

An assignment of the form `x = x op y` uses `x op= y`.

**Why:** The shorter form names `x` once, so the two names cannot drift apart.

```codelaw
match:
  kind: assignment_expression
  has:
    field: left
    kind: identifier
    pattern: $X
  all:
  - has:
      field: right
      kind: binary_expression
      has:
        field: left
        pattern: $X
      all:
      - has:
          field: operator
          regex: ^(\+|-|\*|/|%|\*\*|<<|>>|>>>|&|\||\^)$
      - has:
          field: right
          pattern: $Y
message: Use the compound assignment operator
```

## How to fix

1. Rewrite `x = x + y` as `x += y` (and likewise for the other operators).

## Examples

### JavaScript

#### Don't

```js
total = total + price;
```

#### Do

```js
total += price;
```

### JSX

#### Don't

```jsx
total = total + price;
```

#### Do

```jsx
total += price;
```

### TypeScript

#### Don't

```ts
total = total + price;
```

#### Do

```ts
total += price;
```

### TSX

#### Don't

```tsx
total = total + price;
```

#### Do

```tsx
total += price;
```
