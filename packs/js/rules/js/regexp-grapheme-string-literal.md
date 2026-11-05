---
id: js/regexp-grapheme-string-literal
title: "String literals in a class hold one character each"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a string out of the class into an alternation changes the pattern's structure, which the author decides"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-regexp/grapheme-string-literal]
since: 0.5.0
---

Under the `v` flag, each string of a `\q{…}` class string literal is a single character (one grapheme); longer strings are written as an alternation outside the class.

**Why:** A class reads as a set of characters; a word inside it is easy to miss.

```codelaw
native: js/regexp-grapheme-string-literal
```

## How to fix

1. Move the multi-character string out of the class: `(?:abc|[x])`.

## Examples

### JavaScript

#### Don't

```js
const re = /[\q{abc}x]/v;
```

#### Do

```js
const re = /(?:abc|[x])/v;
```

### JSX

#### Don't

```jsx
const re = /[\q{abc}x]/v;
```

#### Do

```jsx
const re = /(?:abc|[x])/v;
```

### TypeScript

#### Don't

```ts
const re = /[\q{abc}x]/v;
```

#### Do

```ts
const re = /(?:abc|[x])/v;
```

### TSX

#### Don't

```tsx
const re = /[\q{abc}x]/v;
```

#### Do

```tsx
const re = /(?:abc|[x])/v;
```
