---
id: js/no-nested-ternary
title: "No nested conditional expressions"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a nested conditional that is the whole value of a `return` or an arrow's concise body becomes an exact if/return chain; nesting elsewhere (declarations, arguments, parentheses) carries no edit"
stability: preview
ported_from: [eslint/no-nested-ternary, eslint-plugin-sonarjs/no-nested-conditional, eslint-plugin-unicorn/no-nested-ternary]
since: 0.5.0
---

A conditional expression does not contain another one.

**Why:** Nested ternaries are hard to read; an `if` chain or a lookup is clearer.

```codelaw
match:
  any:
    - kind: ternary_expression
      inside: { kind: return_statement, stopBy: neighbor, pattern: $RET }
      all:
        - has: { field: condition, pattern: $A1 }
        - has: { field: consequence, pattern: $X1 }
        - has:
            field: alternative
            kind: ternary_expression
            all:
              - has: { field: condition, pattern: $B1 }
              - has: { field: consequence, pattern: $Y1 }
              - has: { field: alternative, pattern: $Z1 }
    - kind: ternary_expression
      pattern: $T2
      inside: { kind: arrow_function, field: body, stopBy: neighbor }
      all:
        - has: { field: condition, pattern: $A2 }
        - has: { field: consequence, pattern: $X2 }
        - has:
            field: alternative
            kind: ternary_expression
            all:
              - has: { field: condition, pattern: $B2 }
              - has: { field: consequence, pattern: $Y2 }
              - has: { field: alternative, pattern: $Z2 }
    - kind: ternary_expression
      has:
        any:
          - kind: ternary_expression
          - kind: parenthesized_expression
            has: { kind: ternary_expression }
message: "Do not nest conditional expressions"
fix:
  - replace: { target: $RET, with: "if ($A1) return $X1; if ($B1) return $Y1; return $Z1;" }
  - replace: { target: $T2, with: "{ if ($A2) return $X2; if ($B2) return $Y2; return $Z2; }" }
```

## How to fix

1. Rewrite as an `if`/`else` chain or a lookup.

## Examples

### JavaScript

#### Don't

```js
function size(n) {
  return n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
}
```

#### Fixed

```js
function size(n) {
  if (n > 100) return 'large'; if (n > 10) return 'medium'; return 'small';
}
```

#### Do

```js
function size(n) {
  if (n > 100) return 'large';
  if (n > 10) return 'medium';
  return 'small';
}
```

### JSX

#### Don't

```jsx
function size(n) {
  return n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
}
```

#### Fixed

```jsx
function size(n) {
  if (n > 100) return 'large'; if (n > 10) return 'medium'; return 'small';
}
```

#### Do

```jsx
function size(n) {
  if (n > 100) return 'large';
  if (n > 10) return 'medium';
  return 'small';
}
```

### TypeScript

#### Don't

```ts
function size(n) {
  return n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
}
```

#### Fixed

```ts
function size(n) {
  if (n > 100) return 'large'; if (n > 10) return 'medium'; return 'small';
}
```

#### Do

```ts
function size(n) {
  if (n > 100) return 'large';
  if (n > 10) return 'medium';
  return 'small';
}
```

### TSX

#### Don't

```tsx
function size(n) {
  return n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
}
```

#### Fixed

```tsx
function size(n) {
  if (n > 100) return 'large'; if (n > 10) return 'medium'; return 'small';
}
```

#### Do

```tsx
function size(n) {
  if (n > 100) return 'large';
  if (n > 10) return 'medium';
  return 'small';
}
```
