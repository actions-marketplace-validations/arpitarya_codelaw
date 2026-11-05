---
id: js/regexp-no-dupe-disjunctions
title: "No alternative that repeats or is covered by an earlier one"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the alternative changes the pattern's text; check it wasn't a typo for a different one"
stability: preview
ported_from: [eslint-plugin-regexp/no-dupe-disjunctions]
since: 0.5.0
---

An alternative doesn't repeat an earlier one, and a single-character alternative isn't inside an earlier single-character one: `(?:a|b|a)`, `(?:\w|a)`.

**Why:** It can never match, and it slows the backtracking down.

```codelaw
native: js/regexp-no-dupe-disjunctions
```

## How to fix

1. Remove the alternative, or fix the one that was meant.

## Examples

### JavaScript

#### Don't

```js
const re = /(?:ab|cd|ab)/;
```

#### Do

```js
const re = /(?:ab|cd)/;
```

### JSX

#### Don't

```jsx
const re = /(?:ab|cd|ab)/;
```

#### Do

```jsx
const re = /(?:ab|cd)/;
```

### TypeScript

#### Don't

```ts
const re = /(?:ab|cd|ab)/;
```

#### Do

```ts
const re = /(?:ab|cd)/;
```

### TSX

#### Don't

```tsx
const re = /(?:ab|cd|ab)/;
```

#### Do

```tsx
const re = /(?:ab|cd)/;
```
