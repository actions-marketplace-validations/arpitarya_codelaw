---
id: js/regexp-prefer-quantifier
title: "A repeated element is written with a quantifier"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-quantifier]
since: 0.5.0
---

An element repeated in a row is written with a quantifier when that is shorter: `\d\d\d\d` is `\d{4}`.

**Why:** The count is visible at a glance.

```codelaw
native: js/regexp-prefer-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /\d\d\d\d/;
```

#### Do

```js
const re = /\d{4}/;
```

### JSX

#### Don't

```jsx
const re = /\d\d\d\d/;
```

#### Do

```jsx
const re = /\d{4}/;
```

### TypeScript

#### Don't

```ts
const re = /\d\d\d\d/;
```

#### Do

```ts
const re = /\d{4}/;
```

### TSX

#### Don't

```tsx
const re = /\d\d\d\d/;
```

#### Do

```tsx
const re = /\d{4}/;
```
