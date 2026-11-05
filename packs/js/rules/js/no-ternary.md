---
id: js/no-ternary
title: "No ternary"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the expression becomes statements"
stability: preview
ported_from: [eslint/no-ternary]
since: 0.5.0
---

The conditional operator `? :` is not used (taste preset).

**Why:** Some teams find a statement easier to read and to set a breakpoint on than an expression.

```codelaw

match:
  kind: ternary_expression
message: "Write the choice as an `if` statement"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const label = ok ? "yes" : "no";
```

#### Do

```js
let label = "no";
if (ok) {
  label = "yes";
}
```

### JSX

#### Don't

```jsx
const label = ok ? "yes" : "no";
```

#### Do

```jsx
let label = "no";
if (ok) {
  label = "yes";
}
```

### TypeScript

#### Don't

```ts
const label = ok ? "yes" : "no";
```

#### Do

```ts
let label = "no";
if (ok) {
  label = "yes";
}
```

### TSX

#### Don't

```tsx
const label = ok ? "yes" : "no";
```

#### Do

```tsx
let label = "no";
if (ok) {
  label = "yes";
}
```
