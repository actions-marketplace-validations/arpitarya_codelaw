---
id: js/no-unnecessary-array-flat-depth
title: "No flat(1)"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-array-flat-depth]
since: 0.5.0
---

`.flat(1)` is written `.flat()`.

**Why:** 1 is the default depth.

```codelaw
match:
  pattern: "$A.flat(1)"
message: "`1` is the default depth"
fix: "$A.flat()"
```

## Examples

### JavaScript

#### Don't

```js
const list = rows.flat(1);
```

#### Do

```js
const list = rows.flat();
```

### JSX

#### Don't

```jsx
const list = rows.flat(1);
```

#### Do

```jsx
const list = rows.flat();
```

### TypeScript

#### Don't

```ts
const list = rows.flat(1);
```

#### Do

```ts
const list = rows.flat();
```

### TSX

#### Don't

```tsx
const list = rows.flat(1);
```

#### Do

```tsx
const list = rows.flat();
```
