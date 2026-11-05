---
id: js/regexp-no-misleading-unicode-character
title: "No quantifier on part of a character"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-misleading-unicode-character]
since: 0.5.0
---

A quantifier doesn't follow a character built from several code points (an emoji with a modifier, a letter with a combining mark), or an astral character without the `u` flag: it repeats only the last part. Classes are `js/no-misleading-character-class`.

**Why:** `/👍🏻+/` repeats the skin tone, not the thumb.

```codelaw
native: js/regexp-no-misleading-unicode-character
```

## Examples

### JavaScript

#### Don't

```js
const re = /é+/;
```

#### Do

```js
const re = /(?:é)+/;
```

### JSX

#### Don't

```jsx
const re = /é+/;
```

#### Do

```jsx
const re = /(?:é)+/;
```

### TypeScript

#### Don't

```ts
const re = /é+/;
```

#### Do

```ts
const re = /(?:é)+/;
```

### TSX

#### Don't

```tsx
const re = /é+/;
```

#### Do

```tsx
const re = /(?:é)+/;
```
