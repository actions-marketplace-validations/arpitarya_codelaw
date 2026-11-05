---
id: js/regexp-require-unicode-regexp
title: "Regular expressions use the `u` flag"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto when adding `u` can't change what the pattern matches (ASCII only, no `.`, no negation, no `i`); otherwise reported without an edit"
stability: preview
ported_from: [eslint-plugin-regexp/require-unicode-regexp]
since: 0.5.0
---

A regular expression has the `u` (or `v`) flag.

**Why:** It reads code points, not halves of them, and rejects mistaken escapes.

```codelaw
native: js/regexp-require-unicode-regexp
```

## Examples

### JavaScript

#### Don't

```js
const re = /a+/;
```

#### Do

```js
const re = /a+/u;
```

### JSX

#### Don't

```jsx
const re = /a+/;
```

#### Do

```jsx
const re = /a+/u;
```

### TypeScript

#### Don't

```ts
const re = /a+/;
```

#### Do

```ts
const re = /a+/u;
```

### TSX

#### Don't

```tsx
const re = /a+/;
```

#### Do

```tsx
const re = /a+/u;
```
