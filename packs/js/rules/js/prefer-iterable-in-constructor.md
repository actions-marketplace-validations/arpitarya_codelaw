---
id: js/prefer-iterable-in-constructor
title: "new Set(x) rather than new Set([...x])"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterable-in-constructor]
since: 0.5.0
---

A `Set` or `Map` is built from an iterable directly, not from a spread of it.

**Why:** The constructors take any iterable; the spread copies it once more.

```codelaw
match:
  pattern: "new $C([...$X])"
constraints:
  C: { regex: '^(Set|Map|WeakSet|WeakMap)$' }
message: "Pass the iterable directly"
fix: "new $C($X)"
```

## Examples

### JavaScript

#### Don't

```js
const copy = new Set([...ids]);
```

#### Do

```js
const copy = new Set(ids);
```

### JSX

#### Don't

```jsx
const copy = new Set([...ids]);
```

#### Do

```jsx
const copy = new Set(ids);
```

### TypeScript

#### Don't

```ts
const copy = new Set([...ids]);
```

#### Do

```ts
const copy = new Set(ids);
```

### TSX

#### Don't

```tsx
const copy = new Set([...ids]);
```

#### Do

```tsx
const copy = new Set(ids);
```
