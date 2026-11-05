---
id: js/regexp-no-empty-character-class
title: "No character class that matches no character"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which characters the class was meant to hold is the author's to write"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-regexp/no-empty-character-class]
since: 0.5.0
---

A class with elements doesn't cancel them out: `[^\s\S]`, `[\d&&\s]` match nothing. (`[]` is `js/no-empty-character-class`.)

**Why:** The pattern can never match there.

```codelaw
native: js/regexp-no-empty-character-class
```

## How to fix

1. List the characters that were meant.

## Examples

### JavaScript

#### Don't

```js
const re = /a[^\s\S]/;
```

#### Do

```js
const re = /a[\s]/;
```

### JSX

#### Don't

```jsx
const re = /a[^\s\S]/;
```

#### Do

```jsx
const re = /a[\s]/;
```

### TypeScript

#### Don't

```ts
const re = /a[^\s\S]/;
```

#### Do

```ts
const re = /a[\s]/;
```

### TSX

#### Don't

```tsx
const re = /a[^\s\S]/;
```

#### Do

```tsx
const re = /a[\s]/;
```
