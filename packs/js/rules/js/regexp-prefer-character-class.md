---
id: js/regexp-prefer-character-class
title: "Single-character alternatives are a character class"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-character-class]
since: 0.5.0
---

Three or more alternatives of one character each are written as a class: `(?:a|b|c)` is `[abc]`.

**Why:** A class is shorter and doesn't backtrack.

```codelaw
native: js/regexp-prefer-character-class
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?:a|b|c)/;
```

#### Do

```js
const re = /[abc]/;
```

### JSX

#### Don't

```jsx
const re = /(?:a|b|c)/;
```

#### Do

```jsx
const re = /[abc]/;
```

### TypeScript

#### Don't

```ts
const re = /(?:a|b|c)/;
```

#### Do

```ts
const re = /[abc]/;
```

### TSX

#### Don't

```tsx
const re = /(?:a|b|c)/;
```

#### Do

```tsx
const re = /[abc]/;
```
