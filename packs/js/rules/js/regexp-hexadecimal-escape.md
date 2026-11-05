---
id: js/regexp-hexadecimal-escape
title: "Characters up to U+00FF are written as hexadecimal escapes"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/hexadecimal-escape]
since: 0.5.0
---

A character up to U+00FF written as an escape uses `\xHH`, not `\u00HH` or `\u{HH}`.

**Why:** One short form for the Latin-1 range.

```codelaw
native: js/regexp-hexadecimal-escape
```

## Examples

### JavaScript

#### Don't

```js
const re = /\u00e9/;
```

#### Do

```js
const re = /\xE9/;
```

### JSX

#### Don't

```jsx
const re = /\u00e9/;
```

#### Do

```jsx
const re = /\xE9/;
```

### TypeScript

#### Don't

```ts
const re = /\u00e9/;
```

#### Do

```ts
const re = /\xE9/;
```

### TSX

#### Don't

```tsx
const re = /\u00e9/;
```

#### Do

```tsx
const re = /\xE9/;
```
