---
id: js/regexp-use-ignore-case
title: "Both cases of every letter means the `i` flag"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/use-ignore-case]
since: 0.5.0
---

A pattern whose every class lists both cases of the same letters (`[a-zA-Z]`) uses the `i` flag instead.

**Why:** The flag says it once.

```codelaw
native: js/regexp-use-ignore-case
```

## Examples

### JavaScript

#### Don't

```js
const re = /[a-zA-Z]+/;
```

#### Do

```js
const re = /[a-z]+/i;
```

### JSX

#### Don't

```jsx
const re = /[a-zA-Z]+/;
```

#### Do

```jsx
const re = /[a-z]+/i;
```

### TypeScript

#### Don't

```ts
const re = /[a-zA-Z]+/;
```

#### Do

```ts
const re = /[a-z]+/i;
```

### TSX

#### Don't

```tsx
const re = /[a-zA-Z]+/;
```

#### Do

```tsx
const re = /[a-z]+/i;
```
