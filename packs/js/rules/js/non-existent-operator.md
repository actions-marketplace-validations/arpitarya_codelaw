---
id: js/non-existent-operator
title: "No =+, =- or =!"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which operator was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/non-existent-operator]
since: 0.5.0
---

An assignment is not written `x =+ y`, `x =- y` or `x =! y` with no space after `=`.

**Why:** These are a typo for a compound operator or a comparison, and assign the wrong value without an error.

```codelaw

match:
  kind: assignment_expression
  regex: '^[^=]*[^=!<>+\-*/%&|^]=[+\-!](\s|[^=])'
  has: { field: right, any: [{kind: unary_expression}] }
message: "`=+`, `=-` and `=!` assign a unary expression; did you mean `+=`, `-=` or `!=`?"
```

## How to fix

1. Write `+=`, `-=` or `!==`, or put a space after `=` if the unary operator is meant.

## Examples

### JavaScript

#### Don't

```js
total =+ price;
```

#### Do

```js
total += price;
```

### JSX

#### Don't

```jsx
total =+ price;
```

#### Do

```jsx
total += price;
```

### TypeScript

#### Don't

```ts
total =+ price;
```

#### Do

```ts
total += price;
```

### TSX

#### Don't

```tsx
total =+ price;
```

#### Do

```tsx
total += price;
```
