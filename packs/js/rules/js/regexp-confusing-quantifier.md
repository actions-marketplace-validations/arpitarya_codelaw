---
id: js/regexp-confusing-quantifier
title: "No quantifier whose minimum is misleading"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "lowering the minimum to 0 says what the pattern does, but the author may have meant the element to be required"
stability: preview
ported_from: [eslint-plugin-regexp/confusing-quantifier]
since: 0.5.0
---

A quantifier with a minimum above zero doesn't repeat something that can match the empty string: `(a?){2}` reads as two at least, but matches nothing too.

**Why:** The minimum promises characters the pattern doesn't require.

```codelaw
native: js/regexp-confusing-quantifier
```

## How to fix

1. Write the minimum as 0 if empty is meant.
2. Otherwise make what is repeated require a character.

## Examples

### JavaScript

#### Don't

```js
const re = /(a?){2}/;
```

#### Do

```js
const re = /(a?){0,2}/;
```

### JSX

#### Don't

```jsx
const re = /(a?){2}/;
```

#### Do

```jsx
const re = /(a?){0,2}/;
```

### TypeScript

#### Don't

```ts
const re = /(a?){2}/;
```

#### Do

```ts
const re = /(a?){0,2}/;
```

### TSX

#### Don't

```tsx
const re = /(a?){2}/;
```

#### Do

```tsx
const re = /(a?){0,2}/;
```
