---
id: js/regexp-no-lazy-ends
title: "No lazy quantifier at the end of the pattern"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the lazy quantifier at the end always takes its minimum; the edit writes that, which may not be what was meant"
stability: preview
ported_from: [eslint-plugin-regexp/no-lazy-ends]
since: 0.5.0
---

A lazy quantifier isn't the last thing in the pattern: `a+?` at the end always matches one `a`.

**Why:** The quantifier reads as a range but matches its minimum.

```codelaw
native: js/regexp-no-lazy-ends
```

## How to fix

1. Write the minimum it matches, or make it greedy if the range was meant.

## Examples

### JavaScript

#### Don't

```js
const re = /ab+?/;
```

#### Do

```js
const re = /ab/;
```

### JSX

#### Don't

```jsx
const re = /ab+?/;
```

#### Do

```jsx
const re = /ab/;
```

### TypeScript

#### Don't

```ts
const re = /ab+?/;
```

#### Do

```ts
const re = /ab/;
```

### TSX

#### Don't

```tsx
const re = /ab+?/;
```

#### Do

```tsx
const re = /ab/;
```
