---
id: js/regexp-no-useless-two-nums-quantifier
title: "No `{n,n}` quantifier"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-two-nums-quantifier]
since: 0.5.0
---

`{n,n}` is written `{n}`.

**Why:** One number says it.

```codelaw
native: js/regexp-no-useless-two-nums-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /a{2,2}/;
```

#### Do

```js
const re = /a{2}/;
```

### JSX

#### Don't

```jsx
const re = /a{2,2}/;
```

#### Do

```jsx
const re = /a{2}/;
```

### TypeScript

#### Don't

```ts
const re = /a{2,2}/;
```

#### Do

```ts
const re = /a{2}/;
```

### TSX

#### Don't

```tsx
const re = /a{2,2}/;
```

#### Do

```tsx
const re = /a{2}/;
```
