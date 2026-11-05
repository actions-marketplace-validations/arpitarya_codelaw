---
id: js/regexp-no-zero-quantifier
title: "No quantifier that repeats zero times"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the element keeps what the pattern matches, but it was probably meant to be there"
stability: preview
ported_from: [eslint-plugin-regexp/no-zero-quantifier]
since: 0.5.0
---

An element isn't quantified `{0}` or `{0,0}`: it then matches only the empty string.

**Why:** The element might as well not be there, which is rarely what was meant.

```codelaw
native: js/regexp-no-zero-quantifier
```

## How to fix

1. Remove the element, or give the quantifier the count that was meant.

## Examples

### JavaScript

#### Don't

```js
const re = /ab{0}c/;
```

#### Do

```js
const re = /ac/;
```

### JSX

#### Don't

```jsx
const re = /ab{0}c/;
```

#### Do

```jsx
const re = /ac/;
```

### TypeScript

#### Don't

```ts
const re = /ab{0}c/;
```

#### Do

```ts
const re = /ac/;
```

### TSX

#### Don't

```tsx
const re = /ab{0}c/;
```

#### Do

```tsx
const re = /ac/;
```
