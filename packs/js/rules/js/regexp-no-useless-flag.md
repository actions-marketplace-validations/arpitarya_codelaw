---
id: js/regexp-no-useless-flag
title: "No flag that changes nothing"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-flag]
since: 0.5.0
---

A pattern doesn't carry `i` with no cased letter, `m` with no `^` or `$`, or `s` with no `.`.

**Why:** The flag suggests behaviour the pattern doesn't have.

```codelaw
native: js/regexp-no-useless-flag
```

## Examples

### JavaScript

#### Don't

```js
const re = /\d+/i;
```

#### Do

```js
const re = /\d+/;
```

### JSX

#### Don't

```jsx
const re = /\d+/i;
```

#### Do

```jsx
const re = /\d+/;
```

### TypeScript

#### Don't

```ts
const re = /\d+/i;
```

#### Do

```ts
const re = /\d+/;
```

### TSX

#### Don't

```tsx
const re = /\d+/i;
```

#### Do

```tsx
const re = /\d+/;
```
