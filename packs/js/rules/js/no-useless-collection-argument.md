---
id: js/no-useless-collection-argument
title: "No new Set([])"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-collection-argument]
since: 0.5.0
---

A `Set`, `Map`, `WeakSet` or `WeakMap` is not given an empty array, `undefined` or `null`.

**Why:** The collection starts empty either way.

```codelaw

match:
  any: [{ pattern: "new $C([])" }, { pattern: "new $C(undefined)" }, { pattern: "new $C(null)" }]
constraints:
  C: { regex: '^(Set|Map|WeakSet|WeakMap)$' }
message: "An empty argument adds nothing; write `new $C()`"
fix: "new $C()"
```

## Examples

### JavaScript

#### Don't

```js
const seen = new Set([]);
```

#### Do

```js
const seen = new Set();
```

### JSX

#### Don't

```jsx
const seen = new Set([]);
```

#### Do

```jsx
const seen = new Set();
```

### TypeScript

#### Don't

```ts
const seen = new Set([]);
```

#### Do

```ts
const seen = new Set();
```

### TSX

#### Don't

```tsx
const seen = new Set([]);
```

#### Do

```tsx
const seen = new Set();
```
