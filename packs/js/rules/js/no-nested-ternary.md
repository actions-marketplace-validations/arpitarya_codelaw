---
id: js/no-nested-ternary
title: "No nested conditional expressions"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the clearer form — `if`, `switch` or a lookup table — is the author's to choose"
stability: preview
ported_from: [eslint/no-nested-ternary]
since: 0.5.0
---

A conditional expression does not contain another one.

**Why:** Nested ternaries are hard to read; an `if` chain or a lookup is clearer.

```codelaw
match:
  kind: ternary_expression
  has:
    any:
      - kind: ternary_expression
      - kind: parenthesized_expression
        has: { kind: ternary_expression }
message: "Do not nest conditional expressions"
```

## How to fix

1. Rewrite as an `if`/`else` chain or a lookup.

## Examples

### JavaScript

#### Don't

```js
const size = n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
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
const size = n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
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
const size = n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
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
const size = n > 100 ? 'large' : n > 10 ? 'medium' : 'small';
```

#### Do

```tsx
function size(n) {
  if (n > 100) return 'large';
  if (n > 10) return 'medium';
  return 'small';
}
```
