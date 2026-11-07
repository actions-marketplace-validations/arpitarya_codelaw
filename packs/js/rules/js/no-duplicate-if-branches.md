---
id: js/no-duplicate-if-branches
title: "No adjacent if branches with the same body"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "merging the conditions keeps one copy of the body and drops the other's comments; offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-duplicate-if-branches, eslint-plugin-sonarjs/no-duplicated-branches]
since: 0.5.0
---

An `if` and the `else if` right after it do not have the same body; their conditions are joined with `||`.

**Why:** Two copies of one body must be changed together; one condition says the same thing.

```codelaw
utils:
  operand:
    not:
      any:
        - kind: ternary_expression
        - kind: assignment_expression
        - kind: augmented_assignment_expression
        - kind: sequence_expression
        - kind: arrow_function
        - kind: yield_expression
        - kind: binary_expression
          has: { field: operator, regex: '^\?\?$' }
match:
  any:
    - pattern: "if ($A) { $$$X } else if ($B) { $$$X } else $E"
      all: [{ pattern: $ELSE }]
    - pattern: "if ($A) { $$$X } else if ($B) { $$$X }"
      not: { has: { field: alternative, has: { kind: if_statement, has: { field: alternative, kind: else_clause } } } }
      all: [{ pattern: $END }]
constraints:
  A: { matches: operand }
  B: { matches: operand }
message: "This branch repeats the one before it; join the conditions"
fix:
  - replace: { target: $ELSE, with: "if ($A || $B) {\n  $$$X\n} else $E" }
  - replace: { target: $END, with: "if ($A || $B) {\n  $$$X\n}" }
```

## Examples

### JavaScript

#### Don't

```js
if (role === "admin") {
  allow();
} else if (role === "owner") {
  allow();
}
```

#### Do

```js
if (role === "admin" || role === "owner") {
  allow();
}
```


### JSX

#### Don't

```jsx
if (role === "admin") {
  allow();
} else if (role === "owner") {
  allow();
}
```

#### Do

```jsx
if (role === "admin" || role === "owner") {
  allow();
}
```


### TypeScript

#### Don't

```ts
if (role === "admin") {
  allow();
} else if (role === "owner") {
  allow();
}
```

#### Do

```ts
if (role === "admin" || role === "owner") {
  allow();
}
```


### TSX

#### Don't

```tsx
if (role === "admin") {
  allow();
} else if (role === "owner") {
  allow();
}
```

#### Do

```tsx
if (role === "admin" || role === "owner") {
  allow();
}
```
