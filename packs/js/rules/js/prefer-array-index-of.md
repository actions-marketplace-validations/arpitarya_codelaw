---
id: js/prefer-array-index-of
title: "indexOf() rather than findIndex(x => x === v)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-index-of]
since: 0.5.0
---

`.findIndex(x => x === v)` is written `.indexOf(v)`.

**Why:** No callback is needed to look up a value.

```codelaw
match:
  pattern: "$A.findIndex(($X) => $X === $V)"
message: "Use `.indexOf()`"
fix: "$A.indexOf($V)"
```

## Examples

### JavaScript

#### Don't

```js
const i = ids.findIndex((x) => x === id);
```

#### Do

```js
const i = ids.indexOf(id);
```

### JSX

#### Don't

```jsx
const i = ids.findIndex((x) => x === id);
```

#### Do

```jsx
const i = ids.indexOf(id);
```

### TypeScript

#### Don't

```ts
const i = ids.findIndex((x) => x === id);
```

#### Do

```ts
const i = ids.indexOf(id);
```

### TSX

#### Don't

```tsx
const i = ids.findIndex((x) => x === id);
```

#### Do

```tsx
const i = ids.indexOf(id);
```
