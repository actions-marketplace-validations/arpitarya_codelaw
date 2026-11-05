---
id: js/regexp-no-useless-range
title: "No range of one or two characters"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-range]
since: 0.5.0
---

A range in a class covers at least three characters: `[a-a]` is `[a]`, `[a-b]` is `[ab]`.

**Why:** A tiny range hides two plain characters.

```codelaw
native: js/regexp-no-useless-range
```

## Examples

### JavaScript

#### Don't

```js
const re = /[a-bx]/;
```

#### Do

```js
const re = /[abx]/;
```

### JSX

#### Don't

```jsx
const re = /[a-bx]/;
```

#### Do

```jsx
const re = /[abx]/;
```

### TypeScript

#### Don't

```ts
const re = /[a-bx]/;
```

#### Do

```ts
const re = /[abx]/;
```

### TSX

#### Don't

```tsx
const re = /[a-bx]/;
```

#### Do

```tsx
const re = /[abx]/;
```
