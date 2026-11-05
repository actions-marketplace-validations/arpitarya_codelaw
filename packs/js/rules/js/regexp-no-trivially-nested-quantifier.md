---
id: js/regexp-no-trivially-nested-quantifier
title: "No quantifier nested trivially in another"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-trivially-nested-quantifier]
since: 0.5.0
---

A non-capturing group holding one quantified element isn't quantified again when one quantifier says the same: `(?:a+)?` is `a*`, `(?:a{2}){3}` is `a{6}`.

**Why:** Two quantifiers where one does reads as a puzzle.

```codelaw
native: js/regexp-no-trivially-nested-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?:a+)?/;
```

#### Do

```js
const re = /a*/;
```

### JSX

#### Don't

```jsx
const re = /(?:a+)?/;
```

#### Do

```jsx
const re = /a*/;
```

### TypeScript

#### Don't

```ts
const re = /(?:a+)?/;
```

#### Do

```ts
const re = /a*/;
```

### TSX

#### Don't

```tsx
const re = /(?:a+)?/;
```

#### Do

```tsx
const re = /a*/;
```
