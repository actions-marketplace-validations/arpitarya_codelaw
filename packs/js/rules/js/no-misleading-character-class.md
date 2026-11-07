---
id: js/no-misleading-character-class
title: "No character class holding a character made of several code points"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the choices are writing each several-code-point character as an alternative beside the class (the default), or the `u` flag when the class holds astral characters; either changes what the pattern matches, to what it shows"
fix_exception: value
stability: preview
ported_from: [eslint/no-misleading-character-class]
since: 0.1.0
---

A character class contains no character built from several code points — an emoji with a skin-tone
modifier, a flag, a letter with a combining mark — and no astral character without the `u` flag.

**Why:** A class matches one code point at a time, so `[👍🏻]` matches the thumb or the tone, never
the pair.

```codelaw
native: js/no-misleading-character-class
```

## How to fix

1. Use an alternation such as `(?:👍🏻|👍)` for multi-code-point characters.
2. Add the `u` flag for astral characters.

## Examples

### JavaScript

#### Don't

```js
const re = /^[❤️]$/u;
```

#### Do

```js
const re = /^(?:❤️)$/u;
```

### JSX

#### Don't

```jsx
const re = /^[❤️]$/u;
```

#### Do

```jsx
const re = /^(?:❤️)$/u;
```

### TypeScript

#### Don't

```ts
const re = /^[❤️]$/u;
```

#### Do

```ts
const re = /^(?:❤️)$/u;
```

### TSX

#### Don't

```tsx
const re = /^[❤️]$/u;
```

#### Do

```tsx
const re = /^(?:❤️)$/u;
```
