---
id: js/prefer-string-slice
title: "slice() rather than substr()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-slice]
since: 0.5.0
---

`substr` with one argument is written `slice` (the same for one argument).

**Why:** `substr` is deprecated, and `substring` swaps and clamps its arguments in surprising ways.

```codelaw
match:
  pattern: "$S.substr($A)"
message: "Use `.slice()`"
fix: "$S.slice($A)"
```

## Examples

### JavaScript

#### Don't

```js
const ext = name.substr(-3);
```

#### Do

```js
const ext = name.slice(-3);
```

### JSX

#### Don't

```jsx
const ext = name.substr(-3);
```

#### Do

```jsx
const ext = name.slice(-3);
```

### TypeScript

#### Don't

```ts
const ext = name.substr(-3);
```

#### Do

```ts
const ext = name.slice(-3);
```

### TSX

#### Don't

```tsx
const ext = name.substr(-3);
```

#### Do

```tsx
const ext = name.slice(-3);
```
