---
id: js/arrow-body-style
title: "No braces around a single returned expression"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/arrow-body-style, eslint-plugin-sonarjs/arrow-function-convention]
since: 0.5.0
---

An arrow function whose body is only `return expr;` is written with the expression body (the upstream default, `"as-needed"`). An object literal is left alone, since it needs parentheses.

**Why:** The braces and `return` add two lines for nothing.

```codelaw
match:
  kind: arrow_function
  has:
    field: body
    kind: statement_block
    pattern: $B
    has:
      kind: return_statement
      has:
        pattern: $X
        not:
          any:
          - kind: object
          - kind: sequence_expression
    not:
      has:
        nthChild: 2
    all:
    - not:
        has:
          kind: comment
message: 'Return the expression directly: `=> $X`'
fix:
- replace:
    target: $B
    with: $X
```

## Examples

### JavaScript

#### Don't

```js
const double = (x) => {
  return x * 2;
};
```

#### Do

```js
const double = (x) => x * 2;
```

### JSX

#### Don't

```jsx
const double = (x) => {
  return x * 2;
};
```

#### Do

```jsx
const double = (x) => x * 2;
```

### TypeScript

#### Don't

```ts
const double = (x) => {
  return x * 2;
};
```

#### Do

```ts
const double = (x) => x * 2;
```

### TSX

#### Don't

```tsx
const double = (x) => {
  return x * 2;
};
```

#### Do

```tsx
const double = (x) => x * 2;
```
