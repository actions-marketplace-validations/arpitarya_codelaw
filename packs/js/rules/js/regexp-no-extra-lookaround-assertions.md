---
id: js/regexp-no-extra-lookaround-assertions
title: "No lookaround nested at the edge of another of its kind"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-extra-lookaround-assertions]
since: 0.5.0
---

A lookahead that ends with a lookahead, or a lookbehind that starts with a lookbehind, holds its contents directly: `(?=a(?=b))` is `(?=ab)`.

**Why:** The inner assertion adds nothing but brackets.

```codelaw
native: js/regexp-no-extra-lookaround-assertions
```

## Examples

### JavaScript

#### Don't

```js
const re = /x(?=a(?=b))/;
```

#### Do

```js
const re = /x(?=ab)/;
```

### JSX

#### Don't

```jsx
const re = /x(?=a(?=b))/;
```

#### Do

```jsx
const re = /x(?=ab)/;
```

### TypeScript

#### Don't

```ts
const re = /x(?=a(?=b))/;
```

#### Do

```ts
const re = /x(?=ab)/;
```

### TSX

#### Don't

```tsx
const re = /x(?=a(?=b))/;
```

#### Do

```tsx
const re = /x(?=ab)/;
```
