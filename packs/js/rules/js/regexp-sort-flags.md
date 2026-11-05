---
id: js/regexp-sort-flags
title: "Flags are in order"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/sort-flags]
since: 0.5.0
---

A regular expression's flags are in alphabetical order: `gim`, not `mig`.

**Why:** One order makes flags easy to compare.

```codelaw
native: js/regexp-sort-flags
```

## Examples

### JavaScript

#### Don't

```js
const re = /a/mig;
```

#### Do

```js
const re = /a/gim;
```

### JSX

#### Don't

```jsx
const re = /a/mig;
```

#### Do

```jsx
const re = /a/gim;
```

### TypeScript

#### Don't

```ts
const re = /a/mig;
```

#### Do

```ts
const re = /a/gim;
```

### TSX

#### Don't

```tsx
const re = /a/mig;
```

#### Do

```tsx
const re = /a/gim;
```
