---
id: js/regexp-no-octal
title: "No octal escape"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the escape may have been meant as a backreference; the hexadecimal form keeps what it matches"
stability: preview
ported_from: [eslint-plugin-regexp/no-octal]
since: 0.5.0
---

A pattern doesn't use an octal escape such as `\07`; `\0` alone is allowed.

**Why:** It reads like a backreference.

```codelaw
native: js/regexp-no-octal
```

## How to fix

1. Write the character as a hexadecimal escape, or fix the backreference.

## Examples

### JavaScript

#### Don't

```js
const re = /\07/;
```

#### Do

```js
const re = /\x07/;
```

### JSX

#### Don't

```jsx
const re = /\07/;
```

#### Do

```jsx
const re = /\x07/;
```

### TypeScript

#### Don't

```ts
const re = /\07/;
```

#### Do

```ts
const re = /\x07/;
```

### TSX

#### Don't

```tsx
const re = /\07/;
```

#### Do

```tsx
const re = /\x07/;
```
