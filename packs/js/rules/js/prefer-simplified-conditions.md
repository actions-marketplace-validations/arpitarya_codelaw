---
id: js/prefer-simplified-conditions
title: "Push a negation through a group with De Morgan's law when it removes one"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-simplified-conditions]
since: 0.5.0
---

In a condition, `!(!a && !b)` is `a || b`, `!(!a || !b)` is `a && b`, `!(a === b && !c)` is `a !== b || c` and `!(a !== b || !c)` is `a === b && c`: each rewrite removes negations. Only conditions of `if`, `while` and `?:` are rewritten, since there only truthiness is read. Upstream also factors common terms and drops absorbed ones; this rule applies De Morgan's law.

**Why:** Fewer negations are fewer things to keep in mind while reading the test.

```codelaw
utils:
  cond:
    any:
      - inside:
          kind: parenthesized_expression
          inside: { any: [{ kind: if_statement }, { kind: while_statement }, { kind: do_statement }], field: condition }
      - inside: { kind: ternary_expression, field: condition }
match:
  kind: unary_expression
  matches: cond
  any:
    - all:
        - pattern: "!(!$A && !$B)"
        - pattern: $W1
    - all:
        - pattern: "!(!$C || !$D)"
        - pattern: $W2
    - all:
        - pattern: "!($E === $F && !$G)"
        - pattern: $W3
    - all:
        - pattern: "!($H !== $I || !$K)"
        - pattern: $W4
message: "Apply De Morgan's law to drop the negations"
fix:
  - replace: { target: $W1, with: "$A || $B" }
  - replace: { target: $W2, with: "$C && $D" }
  - replace: { target: $W3, with: "$E !== $F || $G" }
  - replace: { target: $W4, with: "$H === $I && $K" }
```

## Examples

### JavaScript

#### Don't

```js
if (!(key === "y" && !editable)) {}
```

#### Do

```js
if (key !== "y" || editable) {}
```


### JSX

#### Don't

```jsx
if (!(key === "y" && !editable)) {}
```

#### Do

```jsx
if (key !== "y" || editable) {}
```


### TypeScript

#### Don't

```ts
if (!(key === "y" && !editable)) {}
```

#### Do

```ts
if (key !== "y" || editable) {}
```


### TSX

#### Don't

```tsx
if (!(key === "y" && !editable)) {}
```

#### Do

```tsx
if (key !== "y" || editable) {}
```
