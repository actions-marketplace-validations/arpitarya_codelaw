---
id: js/regexp-prefer-star-quantifier
title: "`{0,}` is written `*`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-star-quantifier]
since: 0.5.0
---

The quantifier `{0,}` is written `*`.

**Why:** It is the usual form.

```codelaw
native: js/regexp-prefer-star-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /a{0,}/;
```

#### Do

```js
const re = /a*/;
```

### JSX

#### Don't

```jsx
const re = /a{0,}/;
```

#### Do

```jsx
const re = /a*/;
```

### TypeScript

#### Don't

```ts
const re = /a{0,}/;
```

#### Do

```ts
const re = /a*/;
```

### TSX

#### Don't

```tsx
const re = /a{0,}/;
```

#### Do

```tsx
const re = /a*/;
```
