---
id: js/regexp-match-any
title: "Any character is matched with `[\\s\\S]`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/match-any]
since: 0.5.0
---

A class that matches every character is written `[\s\S]`, not `[\d\D]`, `[\w\W]` or `[^]`.

**Why:** One recognisable spelling of “any character”.

```codelaw
native: js/regexp-match-any
```

## Examples

### JavaScript

#### Don't

```js
const re = /a[\d\D]b/;
```

#### Do

```js
const re = /a[\s\S]b/;
```

### JSX

#### Don't

```jsx
const re = /a[\d\D]b/;
```

#### Do

```jsx
const re = /a[\s\S]b/;
```

### TypeScript

#### Don't

```ts
const re = /a[\d\D]b/;
```

#### Do

```ts
const re = /a[\s\S]b/;
```

### TSX

#### Don't

```tsx
const re = /a[\d\D]b/;
```

#### Do

```tsx
const re = /a[\s\S]b/;
```
