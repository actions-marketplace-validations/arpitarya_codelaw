---
id: js/regexp-negation
title: "A negated class of one escape is the negated escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/negation]
since: 0.5.0
---

`[^\d]` is `\D`, `[^\s]` is `\S`, `[^\w]` is `\W`, `[^\p{L}]` is `\P{L}`.

**Why:** The negated escape is shorter and plainer.

```codelaw
native: js/regexp-negation
```

## Examples

### JavaScript

#### Don't

```js
const re = /[^\d]/;
```

#### Do

```js
const re = /\D/;
```

### JSX

#### Don't

```jsx
const re = /[^\d]/;
```

#### Do

```jsx
const re = /\D/;
```

### TypeScript

#### Don't

```ts
const re = /[^\d]/;
```

#### Do

```ts
const re = /\D/;
```

### TSX

#### Don't

```tsx
const re = /[^\d]/;
```

#### Do

```tsx
const re = /\D/;
```
