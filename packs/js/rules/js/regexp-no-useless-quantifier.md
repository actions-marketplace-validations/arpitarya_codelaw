---
id: js/regexp-no-useless-quantifier
title: "No quantifier that does nothing"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-quantifier]
since: 0.5.0
---

A quantifier isn't `{1}`, and doesn't repeat something that matches only the empty string.

**Why:** It reads as repetition and isn't.

```codelaw
native: js/regexp-no-useless-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /ab{1}/;
```

#### Do

```js
const re = /ab/;
```

### JSX

#### Don't

```jsx
const re = /ab{1}/;
```

#### Do

```jsx
const re = /ab/;
```

### TypeScript

#### Don't

```ts
const re = /ab{1}/;
```

#### Do

```ts
const re = /ab/;
```

### TSX

#### Don't

```tsx
const re = /ab{1}/;
```

#### Do

```tsx
const re = /ab/;
```
