---
id: js/prefer-has-check
title: "has() rather than get() !== undefined"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a key whose value is `undefined` now counts as present"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-has-check]
since: 0.5.0
---

`map.get(k) !== undefined` is written `map.has(k)`.

**Why:** `has` says what is asked; a key stored with the value `undefined` is present.

```codelaw
match:
  pattern: "$M.get($K) !== undefined"
message: "Use `.has()`"
fix: "$M.has($K)"
```

## Examples

### JavaScript

#### Don't

```js
if (cache.get(key) !== undefined) {
  hit();
}
```

#### Do

```js
if (cache.has(key)) {
  hit();
}
```

### JSX

#### Don't

```jsx
if (cache.get(key) !== undefined) {
  hit();
}
```

#### Do

```jsx
if (cache.has(key)) {
  hit();
}
```

### TypeScript

#### Don't

```ts
if (cache.get(key) !== undefined) {
  hit();
}
```

#### Do

```ts
if (cache.has(key)) {
  hit();
}
```

### TSX

#### Don't

```tsx
if (cache.get(key) !== undefined) {
  hit();
}
```

#### Do

```tsx
if (cache.has(key)) {
  hit();
}
```
