---
id: js/regexp-unicode-property
title: "Unicode properties in their usual form"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/unicode-property]
since: 0.5.0
---

A property escape drops the `General_Category=` (or `gc=`) key, and names a script by its long name: `\p{gc=L}` is `\p{L}`, `\p{sc=Grek}` is `\p{sc=Greek}`.

**Why:** One spelling of each property is easier to search for.

```codelaw
native: js/regexp-unicode-property
```

## Examples

### JavaScript

#### Don't

```js
const re = /\p{sc=Grek}/u;
```

#### Do

```js
const re = /\p{sc=Greek}/u;
```

### JSX

#### Don't

```jsx
const re = /\p{sc=Grek}/u;
```

#### Do

```jsx
const re = /\p{sc=Greek}/u;
```

### TypeScript

#### Don't

```ts
const re = /\p{sc=Grek}/u;
```

#### Do

```ts
const re = /\p{sc=Greek}/u;
```

### TSX

#### Don't

```tsx
const re = /\p{sc=Grek}/u;
```

#### Do

```tsx
const re = /\p{sc=Greek}/u;
```
