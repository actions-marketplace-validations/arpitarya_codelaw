---
id: js/no-nested-assignment
title: "No assignment inside an expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the statement is a structural edit"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-nested-assignment]
since: 0.5.0
---

An assignment is a statement of its own, not part of a larger expression (loop headers and `while ((m = re.exec(s)))` are allowed).

**Why:** An assignment hidden in an expression is easily read as a comparison.

```codelaw
match:
  kind: assignment_expression
  not:
    inside:
      any:
      - kind: expression_statement
      - kind: for_statement
      - kind: sequence_expression
        inside:
          kind: for_statement
      - kind: parenthesized_expression
        inside:
          any:
          - kind: while_statement
          - kind: do_statement
  all:
  - not:
      inside:
        kind: arrow_function
        field: body
message: Assign on its own line, then use the variable
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const total = (count = count + 1) * 2;
```

#### Do

```js
count = count + 1;
const total = count * 2;
```

### JSX

#### Don't

```jsx
const total = (count = count + 1) * 2;
```

#### Do

```jsx
count = count + 1;
const total = count * 2;
```

### TypeScript

#### Don't

```ts
const total = (count = count + 1) * 2;
```

#### Do

```ts
count = count + 1;
const total = count * 2;
```

### TSX

#### Don't

```tsx
const total = (count = count + 1) * 2;
```

#### Do

```tsx
count = count + 1;
const total = count * 2;
```
