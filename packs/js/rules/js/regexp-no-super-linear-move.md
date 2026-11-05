---
id: js/regexp-no-super-linear-move
title: "No unanchored pattern that starts with an unbounded quantifier"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "an anchor or a different quantifier changes what the pattern finds; the author decides"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-regexp/no-super-linear-move]
since: 0.5.0
---

A pattern that isn't anchored or sticky doesn't start with an unbounded quantifier followed by more: `/\s+$/` retries the quantifier at every position.

**Why:** A failed search takes quadratic time on a long run of those characters.

```codelaw
native: js/regexp-no-super-linear-move
```

## How to fix

1. Anchor the pattern, or rewrite it so the quantifier can't restart at every position.

## Examples

### JavaScript

#### Don't

```js
const re = /\s+$/;
```

#### Do

```js
const re = /^\s+$/;
```

### JSX

#### Don't

```jsx
const re = /\s+$/;
```

#### Do

```jsx
const re = /^\s+$/;
```

### TypeScript

#### Don't

```ts
const re = /\s+$/;
```

#### Do

```ts
const re = /^\s+$/;
```

### TSX

#### Don't

```tsx
const re = /\s+$/;
```

#### Do

```tsx
const re = /^\s+$/;
```
