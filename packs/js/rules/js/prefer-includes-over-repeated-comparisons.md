---
id: js/prefer-includes-over-repeated-comparisons
title: "includes() rather than three or more === tests of one value"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`includes` evaluates every candidate and treats `NaN` as equal to itself; the rewrite is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-includes-over-repeated-comparisons]
since: 0.5.0
---

A value compared with `===` against three candidates joined by `||` is tested with `[a, b, c].includes(value)`.

**Why:** The list names the candidates once and the value once.

```codelaw
utils:
  operand:
    not:
      any:
        - kind: ternary_expression
        - kind: assignment_expression
        - kind: sequence_expression
        - kind: arrow_function
        - kind: binary_expression
          has: { field: operator, regex: '^(\|\||&&|\?\?)$' }
match:
  pattern: "$X === $A || $X === $B || $X === $C"
  not: { inside: { kind: binary_expression, has: { field: operator, regex: '^\|\|$' } } }
constraints:
  A: { matches: operand }
  B: { matches: operand }
  C: { matches: operand }
message: "Test the candidates with `includes`"
fix: "[$A, $B, $C].includes($X)"
```

## Examples

### JavaScript

#### Don't

```js
if (key === "a" || key === "d" || key === "w") move(key);
```

#### Do

```js
if (["a", "d", "w"].includes(key)) move(key);
```


### JSX

#### Don't

```jsx
if (key === "a" || key === "d" || key === "w") move(key);
```

#### Do

```jsx
if (["a", "d", "w"].includes(key)) move(key);
```


### TypeScript

#### Don't

```ts
if (key === "a" || key === "d" || key === "w") move(key);
```

#### Do

```ts
if (["a", "d", "w"].includes(key)) move(key);
```


### TSX

#### Don't

```tsx
if (key === "a" || key === "d" || key === "w") move(key);
```

#### Do

```tsx
if (["a", "d", "w"].includes(key)) move(key);
```
