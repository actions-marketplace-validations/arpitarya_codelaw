---
id: js/regexp-no-contradiction-with-assertion
title: "No element the assertion before it contradicts"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the element or its quantifier says what the pattern does, which may not be what was meant"
stability: preview
ported_from: [eslint-plugin-regexp/no-contradiction-with-assertion]
since: 0.5.0
---

An optional element right after a `\b` or `\B` either can never match there or must always match: `a\b-?a` always takes the `-`.

**Why:** The quantifier claims a choice the assertion has already made.

```codelaw
native: js/regexp-no-contradiction-with-assertion
```

## How to fix

1. Make the element required, or remove it.

## Examples

### JavaScript

#### Don't

```js
const re = /a\b-?a/;
```

#### Do

```js
const re = /a\b-a/;
```

### JSX

#### Don't

```jsx
const re = /a\b-?a/;
```

#### Do

```jsx
const re = /a\b-a/;
```

### TypeScript

#### Don't

```ts
const re = /a\b-?a/;
```

#### Do

```ts
const re = /a\b-a/;
```

### TSX

#### Don't

```tsx
const re = /a\b-?a/;
```

#### Do

```tsx
const re = /a\b-a/;
```
