---
id: js/regexp-simplify-set-operations
title: "Set operations in their simplest form"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/simplify-set-operations]
since: 0.5.0
---

Under the `v` flag, `[a&&[^b]]` is `[a--[b]]`, `[a--[^b]]` is `[a&&[b]]`, and `[^[^a][^b]]` is `[[a]&&[b]]`.

**Why:** Fewer negations are easier to read.

```codelaw
native: js/regexp-simplify-set-operations
```

## Examples

### JavaScript

#### Don't

```js
const re = /[\w&&[^\d]]/v;
```

#### Do

```js
const re = /[\w--[\d]]/v;
```

### JSX

#### Don't

```jsx
const re = /[\w&&[^\d]]/v;
```

#### Do

```jsx
const re = /[\w--[\d]]/v;
```

### TypeScript

#### Don't

```ts
const re = /[\w&&[^\d]]/v;
```

#### Do

```ts
const re = /[\w--[\d]]/v;
```

### TSX

#### Don't

```tsx
const re = /[\w&&[^\d]]/v;
```

#### Do

```tsx
const re = /[\w--[\d]]/v;
```
