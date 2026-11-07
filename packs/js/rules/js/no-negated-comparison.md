---
id: js/no-negated-comparison
title: "No negated equality comparison"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-negated-comparison]
since: 0.5.0
---

An equality comparison is not negated as a whole: `!(a === b)` is `a !== b`, and `!(a !== b)` is `a === b`.

**Why:** The operator says it directly; the negation and its parentheses make the reader invert it.

```codelaw
match:
  kind: unary_expression
  any:
    - all: [{ pattern: "!($A === $B)" }, { pattern: $SEQ }]
    - all: [{ pattern: "!($A !== $B)" }, { pattern: $SNE }]
    - all: [{ pattern: "!($A == $B)" }, { pattern: $LEQ }]
    - all: [{ pattern: "!($A != $B)" }, { pattern: $LNE }]
  inside:
    any:
      - kind: parenthesized_expression
      - kind: return_statement
      - kind: variable_declarator
      - kind: arguments
      - kind: expression_statement
      - kind: arrow_function
      - kind: assignment_expression
      - kind: ternary_expression
      - kind: binary_expression
        has: { field: operator, regex: '^(&&|\|\|)$' }
message: "Use the opposite operator"
fix:
  - replace: { target: $SEQ, with: "$A !== $B" }
  - replace: { target: $SNE, with: "$A === $B" }
  - replace: { target: $LEQ, with: "$A != $B" }
  - replace: { target: $LNE, with: "$A == $B" }
```

## Examples

### JavaScript

#### Don't

```js
if (!(status === "done")) retry();
```

#### Do

```js
if (status !== "done") retry();
```


### JSX

#### Don't

```jsx
if (!(status === "done")) retry();
```

#### Do

```jsx
if (status !== "done") retry();
```


### TypeScript

#### Don't

```ts
if (!(status === "done")) retry();
```

#### Do

```ts
if (status !== "done") retry();
```


### TSX

#### Don't

```tsx
if (!(status === "done")) retry();
```

#### Do

```tsx
if (status !== "done") retry();
```
