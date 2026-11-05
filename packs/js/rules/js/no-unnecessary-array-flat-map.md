---
id: js/no-unnecessary-array-flat-map
title: "flat() rather than flatMap(x => x)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-array-flat-map]
since: 0.5.0
---

`.flatMap(x => x)` is written `.flat()`.

**Why:** The identity callback adds nothing.

```codelaw
match:
  pattern: "$A.flatMap(($X) => $X)"
message: "Use `.flat()`"
fix: "$A.flat()"
```

## Examples

### JavaScript

#### Don't

```js
const all = groups.flatMap((g) => g);
```

#### Do

```js
const all = groups.flat();
```

### JSX

#### Don't

```jsx
const all = groups.flatMap((g) => g);
```

#### Do

```jsx
const all = groups.flat();
```

### TypeScript

#### Don't

```ts
const all = groups.flatMap((g) => g);
```

#### Do

```ts
const all = groups.flat();
```

### TSX

#### Don't

```tsx
const all = groups.flatMap((g) => g);
```

#### Do

```tsx
const all = groups.flat();
```
