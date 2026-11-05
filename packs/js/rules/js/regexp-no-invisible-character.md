---
id: js/regexp-no-invisible-character
title: "No invisible character written raw"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-invisible-character]
since: 0.5.0
---

Invisible characters — a tab, a no-break space, a zero-width space — are written as escapes in a pattern.

**Why:** A reader can't see them, so can't tell what the pattern matches.

```codelaw
native: js/regexp-no-invisible-character
```

## Examples

### JavaScript

#### Don't

```js
const re = /a b/;
```

#### Do

```js
const re = /a\xA0b/;
```

### JSX

#### Don't

```jsx
const re = /a b/;
```

#### Do

```jsx
const re = /a\xA0b/;
```

### TypeScript

#### Don't

```ts
const re = /a b/;
```

#### Do

```ts
const re = /a\xA0b/;
```

### TSX

#### Don't

```tsx
const re = /a b/;
```

#### Do

```tsx
const re = /a\xA0b/;
```
