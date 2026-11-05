---
id: js/prefer-array-from-map
title: "Array.from(x, f) rather than Array.from(x).map(f)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the mapper receives `(value, index)` but not the array as its third argument"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-from-map]
since: 0.5.0
---

`Array.from(x).map(f)` is written `Array.from(x, f)`.

**Why:** One array instead of two.

```codelaw
match:
  pattern: "Array.from($X).map($F)"
message: "Pass the mapper to `Array.from()`"
fix: "Array.from($X, $F)"
```

## Examples

### JavaScript

#### Don't

```js
const ids = Array.from(nodes).map((n) => n.id);
```

#### Do

```js
const ids = Array.from(nodes, (n) => n.id);
```

### JSX

#### Don't

```jsx
const ids = Array.from(nodes).map((n) => n.id);
```

#### Do

```jsx
const ids = Array.from(nodes, (n) => n.id);
```

### TypeScript

#### Don't

```ts
const ids = Array.from(nodes).map((n) => n.id);
```

#### Do

```ts
const ids = Array.from(nodes, (n) => n.id);
```

### TSX

#### Don't

```tsx
const ids = Array.from(nodes).map((n) => n.id);
```

#### Do

```tsx
const ids = Array.from(nodes, (n) => n.id);
```
