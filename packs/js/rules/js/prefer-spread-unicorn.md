---
id: js/prefer-spread-unicorn
title: "[...x] rather than Array.from(x)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Array.from` accepts array-likes (an object with `length`) that spread rejects"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-spread]
since: 0.5.0
---

`Array.from(x)` with one argument is written `[...x]`. Upstream also covers `concat` and `slice()`; this port covers `Array.from`.

**Why:** The spread is shorter and reads as "the items of x".

```codelaw
match:
  pattern: "Array.from($X)"
message: "Use `[...$X]`"
fix: "[...$X]"
```

## Examples

### JavaScript

#### Don't

```js
const list = Array.from(set);
```

#### Do

```js
const list = [...set];
```

### JSX

#### Don't

```jsx
const list = Array.from(set);
```

#### Do

```jsx
const list = [...set];
```

### TypeScript

#### Don't

```ts
const list = Array.from(set);
```

#### Do

```ts
const list = [...set];
```

### TSX

#### Don't

```tsx
const list = Array.from(set);
```

#### Do

```tsx
const list = [...set];
```
