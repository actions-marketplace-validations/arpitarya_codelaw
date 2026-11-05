---
id: js/elseif-without-else
title: "An else if chain ends with else"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the final branch does is the author's call"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/elseif-without-else]
since: 0.5.0
---

An `if … else if` chain ends with a final `else`.

**Why:** The last `else` says what happens in the remaining case, instead of leaving the reader to wonder.

```codelaw

match:
  kind: if_statement
  inside: { kind: else_clause }
  not: { has: { field: alternative, regex: "." } }
message: "End the `else if` chain with an `else`, even if it only throws or comments why nothing happens"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Do

```js
if (a) {
  one();
} else if (b) {
  two();
} else {
  none();
}
```

### JSX

#### Don't

```jsx
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Do

```jsx
if (a) {
  one();
} else if (b) {
  two();
} else {
  none();
}
```

### TypeScript

#### Don't

```ts
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Do

```ts
if (a) {
  one();
} else if (b) {
  two();
} else {
  none();
}
```

### TSX

#### Don't

```tsx
if (a) {
  one();
} else if (b) {
  two();
}
```

#### Do

```tsx
if (a) {
  one();
} else if (b) {
  two();
} else {
  none();
}
```
