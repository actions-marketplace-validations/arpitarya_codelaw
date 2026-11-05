---
id: js/regexp-no-useless-lazy
title: "No lazy quantifier that can't be lazy"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-lazy]
since: 0.5.0
---

A quantifier isn't lazy when it repeats a fixed number of times (`a{3}?`) or what follows can't match what it repeats (`\d+?x`).

**Why:** Lazy and greedy match the same there.

```codelaw
native: js/regexp-no-useless-lazy
```

## Examples

### JavaScript

#### Don't

```js
const re = /\d+?x/;
```

#### Do

```js
const re = /\d+x/;
```

### JSX

#### Don't

```jsx
const re = /\d+?x/;
```

#### Do

```jsx
const re = /\d+x/;
```

### TypeScript

#### Don't

```ts
const re = /\d+?x/;
```

#### Do

```ts
const re = /\d+x/;
```

### TSX

#### Don't

```tsx
const re = /\d+?x/;
```

#### Do

```tsx
const re = /\d+x/;
```
