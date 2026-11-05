---
id: js/regexp-no-empty-alternative
title: "No empty alternative"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "making the group optional matches the same strings, but a capturing group then captures nothing instead of the empty string"
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-alternative]
since: 0.5.0
---

A disjunction has no empty alternative: `(?:a|b|)` is written `(?:a|b)?`.

**Why:** An empty alternative is easy to miss and is often a stray `|`.

```codelaw
native: js/regexp-no-empty-alternative
```

## How to fix

1. Make the group optional with `?`, or remove the stray `|`.

## Examples

### JavaScript

#### Don't

```js
const re = /(?:a|b|)c/;
```

#### Do

```js
const re = /(?:a|b)?c/;
```

### JSX

#### Don't

```jsx
const re = /(?:a|b|)c/;
```

#### Do

```jsx
const re = /(?:a|b)?c/;
```

### TypeScript

#### Don't

```ts
const re = /(?:a|b|)c/;
```

#### Do

```ts
const re = /(?:a|b)?c/;
```

### TSX

#### Don't

```tsx
const re = /(?:a|b|)c/;
```

#### Do

```tsx
const re = /(?:a|b)?c/;
```
