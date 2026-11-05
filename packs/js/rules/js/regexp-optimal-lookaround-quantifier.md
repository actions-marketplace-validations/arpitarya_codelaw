---
id: js/regexp-optimal-lookaround-quantifier
title: "No ranged quantifier at the edge of a lookaround"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the quantifier's minimum gives the same result; the edit is offered since the range may have been meant elsewhere"
stability: preview
ported_from: [eslint-plugin-regexp/optimal-lookaround-quantifier]
since: 0.5.0
---

The last element of a lookahead (or the first of a lookbehind) isn't a ranged quantifier: in `(?=ab+)` only the first `b` matters.

**Why:** The range is checked, never used.

```codelaw
native: js/regexp-optimal-lookaround-quantifier
```

## How to fix

1. Write the minimum.

## Examples

### JavaScript

#### Don't

```js
const re = /x(?=ab+)/;
```

#### Do

```js
const re = /x(?=ab)/;
```

### JSX

#### Don't

```jsx
const re = /x(?=ab+)/;
```

#### Do

```jsx
const re = /x(?=ab)/;
```

### TypeScript

#### Don't

```ts
const re = /x(?=ab+)/;
```

#### Do

```ts
const re = /x(?=ab)/;
```

### TSX

#### Don't

```tsx
const re = /x(?=ab+)/;
```

#### Do

```tsx
const re = /x(?=ab)/;
```
