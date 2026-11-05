---
id: js/regexp-prefer-unicode-codepoint-escapes
title: "An astral character is one code point escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-unicode-codepoint-escapes]
since: 0.5.0
---

With the `u` or `v` flag, a surrogate pair written `\uD83D\uDE00` is `\u{1F600}`.

**Why:** One escape for one character.

```codelaw
native: js/regexp-prefer-unicode-codepoint-escapes
```

## Examples

### JavaScript

#### Don't

```js
const re = /\uD83D\uDE00/u;
```

#### Do

```js
const re = /\u{1F600}/u;
```

### JSX

#### Don't

```jsx
const re = /\uD83D\uDE00/u;
```

#### Do

```jsx
const re = /\u{1F600}/u;
```

### TypeScript

#### Don't

```ts
const re = /\uD83D\uDE00/u;
```

#### Do

```ts
const re = /\u{1F600}/u;
```

### TSX

#### Don't

```tsx
const re = /\uD83D\uDE00/u;
```

#### Do

```tsx
const re = /\u{1F600}/u;
```
