---
id: js/regexp-no-non-standard-flag
title: "Only standard regular expression flags"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the flag keeps the pattern standard, but which flag was meant may be another"
stability: preview
ported_from: [eslint-plugin-regexp/no-non-standard-flag]
since: 0.5.0
---

A regular expression uses only the flags the language defines: `d`, `g`, `i`, `m`, `s`, `u`, `v`, `y`.

**Why:** Other flags are engine extensions or typos.

```codelaw
native: js/regexp-no-non-standard-flag
```

## How to fix

1. Replace the flag with the standard one that was meant, or remove it.

## Examples

### JavaScript

#### Don't

```js
const re = new RegExp('a', 'x');
```

#### Do

```js
const re = new RegExp('a', '');
```

### JSX

#### Don't

```jsx
const re = new RegExp('a', 'x');
```

#### Do

```jsx
const re = new RegExp('a', '');
```

### TypeScript

#### Don't

```ts
const re = new RegExp('a', 'x');
```

#### Do

```ts
const re = new RegExp('a', '');
```

### TSX

#### Don't

```tsx
const re = new RegExp('a', 'x');
```

#### Do

```tsx
const re = new RegExp('a', '');
```
