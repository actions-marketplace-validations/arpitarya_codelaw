---
id: js/prefer-array-flat-map
title: "flatMap() rather than map().flat()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-flat-map]
since: 0.5.0
---

`.map(f).flat()` is written `.flatMap(f)`.

**Why:** One pass instead of two, and no intermediate array.

```codelaw
match:
  pattern: "$A.map($F).flat()"
message: "Use `.flatMap()`"
fix: "$A.flatMap($F)"
```

## Examples

### JavaScript

#### Don't

```js
const tags = posts.map((p) => p.tags).flat();
```

#### Do

```js
const tags = posts.flatMap((p) => p.tags);
```

### JSX

#### Don't

```jsx
const tags = posts.map((p) => p.tags).flat();
```

#### Do

```jsx
const tags = posts.flatMap((p) => p.tags);
```

### TypeScript

#### Don't

```ts
const tags = posts.map((p) => p.tags).flat();
```

#### Do

```ts
const tags = posts.flatMap((p) => p.tags);
```

### TSX

#### Don't

```tsx
const tags = posts.map((p) => p.tags).flat();
```

#### Do

```tsx
const tags = posts.flatMap((p) => p.tags);
```
