---
id: js/regexp-require-unicode-sets-regexp
title: "Regular expressions use the `v` flag"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto when switching to `v` can't change what the pattern matches; otherwise reported without an edit"
stability: preview
ported_from: [eslint-plugin-regexp/require-unicode-sets-regexp]
since: 0.5.0
---

A regular expression has the `v` flag, the stricter successor of `u`.

**Why:** It adds set operations and rejects more mistakes.

```codelaw
native: js/regexp-require-unicode-sets-regexp
```

## Examples

### JavaScript

#### Don't

```js
const re = /a+/u;
```

#### Do

```js
const re = /a+/v;
```

### JSX

#### Don't

```jsx
const re = /a+/u;
```

#### Do

```jsx
const re = /a+/v;
```

### TypeScript

#### Don't

```ts
const re = /a+/u;
```

#### Do

```ts
const re = /a+/v;
```

### TSX

#### Don't

```tsx
const re = /a+/u;
```

#### Do

```tsx
const re = /a+/v;
```
