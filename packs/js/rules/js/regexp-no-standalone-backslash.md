---
id: js/regexp-no-standalone-backslash
title: "No backslash that stands for itself"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-standalone-backslash]
since: 0.5.0
---

A `\c` not followed by a letter is written `\\c`: without the `u` flag the backslash stands for itself.

**Why:** It reads as a broken control escape.

```codelaw
native: js/regexp-no-standalone-backslash
```

## Examples

### JavaScript

#### Don't

```js
const re = /\c1/;
```

#### Do

```js
const re = /\\c1/;
```

### JSX

#### Don't

```jsx
const re = /\c1/;
```

#### Do

```jsx
const re = /\\c1/;
```

### TypeScript

#### Don't

```ts
const re = /\c1/;
```

#### Do

```ts
const re = /\\c1/;
```

### TSX

#### Don't

```tsx
const re = /\c1/;
```

#### Do

```tsx
const re = /\\c1/;
```
