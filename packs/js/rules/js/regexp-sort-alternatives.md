---
id: js/regexp-sort-alternatives
title: "Alternatives whose order doesn't matter are sorted"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/sort-alternatives]
since: 0.5.0
---

Alternatives of plain letters and digits, none a prefix of another, are in sorted order.

**Why:** A sorted list is easier to scan and to diff.

```codelaw
native: js/regexp-sort-alternatives
```

## Examples

### JavaScript

#### Don't

```js
const re = /(?:pear|apple|fig)/;
```

#### Do

```js
const re = /(?:apple|fig|pear)/;
```

### JSX

#### Don't

```jsx
const re = /(?:pear|apple|fig)/;
```

#### Do

```jsx
const re = /(?:apple|fig|pear)/;
```

### TypeScript

#### Don't

```ts
const re = /(?:pear|apple|fig)/;
```

#### Do

```ts
const re = /(?:apple|fig|pear)/;
```

### TSX

#### Don't

```tsx
const re = /(?:pear|apple|fig)/;
```

#### Do

```tsx
const re = /(?:apple|fig|pear)/;
```
