---
id: js/regexp-no-obscure-range
title: "No obscure character range"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-obscure-range]
since: 0.5.0
---

A range in a class runs between two digits, two lowercase or two uppercase letters, or two escapes: `[!-#]` and `[A-z]` hide what they hold.

**Why:** `[A-z]` holds `[`, `\`, `]`, `^`, `_` and a backtick too.

```codelaw
native: js/regexp-no-obscure-range
```

## How to fix

1. List the characters, or use escapes for both ends.

## Examples

### JavaScript

#### Don't

```js
const re = /[A-z]/;
```

#### Do

```js
const re = /[\x41-\x7A]/;
```

### JSX

#### Don't

```jsx
const re = /[A-z]/;
```

#### Do

```jsx
const re = /[\x41-\x7A]/;
```

### TypeScript

#### Don't

```ts
const re = /[A-z]/;
```

#### Do

```ts
const re = /[\x41-\x7A]/;
```

### TSX

#### Don't

```tsx
const re = /[A-z]/;
```

#### Do

```tsx
const re = /[\x41-\x7A]/;
```
