---
id: js/regexp-no-super-linear-backtracking
title: "No exponential backtracking"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto when one alternative of a repeated alternation matches a subset of another (it is removed, the language is the same); a repeated sequence with an unbounded quantifier is reported without an edit"
stability: preview
ported_from: [eslint-plugin-regexp/no-super-linear-backtracking]
since: 0.5.0
---

A repeated group can't match the same text in many ways: `(?:\w|\d)*` (alternatives overlap) or `(?:a+b?)*` (one run splits many ways).

**Why:** A failing match on a long input can take exponential time — a denial of service.

```codelaw
native: js/regexp-no-super-linear-backtracking
```

## How to fix

1. Remove the overlap between the alternatives, or make the parts of the repeated sequence exclusive.

## Examples

### JavaScript

#### Don't

```js
const re = /(?:\w|\d)*x/;
```

#### Do

```js
const re = /(?:\w)*x/;
```

### JSX

#### Don't

```jsx
const re = /(?:\w|\d)*x/;
```

#### Do

```jsx
const re = /(?:\w)*x/;
```

### TypeScript

#### Don't

```ts
const re = /(?:\w|\d)*x/;
```

#### Do

```ts
const re = /(?:\w)*x/;
```

### TSX

#### Don't

```tsx
const re = /(?:\w|\d)*x/;
```

#### Do

```tsx
const re = /(?:\w)*x/;
```
