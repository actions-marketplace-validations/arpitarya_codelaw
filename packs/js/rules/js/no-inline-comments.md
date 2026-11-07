---
id: js/no-inline-comments
title: "Comments on their own line"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a comment after code may describe that one line; above it, it can read as describing what follows"
stability: preview
ported_from: [eslint/no-inline-comments]
since: 0.5.0
---

A line comment does not share a line with code (taste preset). Directive comments (`eslint-`, `codelaw-`, `@ts-`) are left alone.

**Why:** A comment above the code is easier to read and survives line edits.

```codelaw

match:
  kind: comment
  regex: '^//'
  not: { regex: '^//\s*(eslint|codelaw|@ts-|prettier)' }
  follows: { regex: ".", pattern: $P }
  inside: { kind: program, stopBy: end }
message: "Put the comment on its own line"
fix:
  - move: { before: $P }
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const max = 3; // retries
```

#### Do

```js
// retries
const max = 3;
```

### JSX

#### Don't

```jsx
const max = 3; // retries
```

#### Do

```jsx
// retries
const max = 3;
```

### TypeScript

#### Don't

```ts
const max = 3; // retries
```

#### Do

```ts
// retries
const max = 3;
```

### TSX

#### Don't

```tsx
const max = 3; // retries
```

#### Do

```tsx
// retries
const max = 3;
```
