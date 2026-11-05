---
id: js/regexp-prefer-plus-quantifier
title: "`{1,}` is written `+`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-plus-quantifier]
since: 0.5.0
---

The quantifier `{1,}` is written `+`.

**Why:** It is the usual form.

```codelaw
native: js/regexp-prefer-plus-quantifier
```

## Examples

### JavaScript

#### Don't

```js
const re = /a{1,}/;
```

#### Do

```js
const re = /a+/;
```

### JSX

#### Don't

```jsx
const re = /a{1,}/;
```

#### Do

```jsx
const re = /a+/;
```

### TypeScript

#### Don't

```ts
const re = /a{1,}/;
```

#### Do

```ts
const re = /a+/;
```

### TSX

#### Don't

```tsx
const re = /a{1,}/;
```

#### Do

```tsx
const re = /a+/;
```
