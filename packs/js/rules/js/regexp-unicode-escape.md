---
id: js/regexp-unicode-escape
title: "Unicode escapes use the code point form"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/unicode-escape]
since: 0.5.0
---

With the `u` or `v` flag, a character above U+00FF written as an escape uses `\u{…}`, not `\uXXXX`.

**Why:** One escape form for every code point.

```codelaw
native: js/regexp-unicode-escape
```

## Examples

### JavaScript

#### Don't

```js
const re = /\u2028/u;
```

#### Do

```js
const re = /\u{2028}/u;
```

### JSX

#### Don't

```jsx
const re = /\u2028/u;
```

#### Do

```jsx
const re = /\u{2028}/u;
```

### TypeScript

#### Don't

```ts
const re = /\u2028/u;
```

#### Do

```ts
const re = /\u{2028}/u;
```

### TSX

#### Don't

```tsx
const re = /\u2028/u;
```

#### Do

```tsx
const re = /\u{2028}/u;
```
