---
id: js/prefer-get-or-insert-computed
title: "`getOrInsertComputed()` for a value that has to be computed"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-get-or-insert-computed]
since: 0.5.0
---

`map.getOrInsert(key, value)` evaluates `value` every time, also when the key is present. When the value calls, constructs, assigns or awaits something, the call is reported at the value and rewritten as `getOrInsertComputed(key, () => value)` — or `getOrInsertComputed(key, compute)` when the value is `compute(key)`. A value with `await`, `yield`, a direct `eval()` or a comment is reported without a fix.

**Why:** The value is thrown away whenever the key exists; a callback runs only when it is needed.

```codelaw
native: js/prefer-get-or-insert-computed
```

## Examples

### JavaScript

#### Don't

```js
cache.getOrInsert(id, load(id));
groups.getOrInsert(name, createGroup());
```

#### Do

```js
cache.getOrInsertComputed(id, load);
groups.getOrInsert(name, []);
```

#### Fixed

```js
cache.getOrInsertComputed(id, load);
groups.getOrInsertComputed(name, () => createGroup());
```


### JSX

#### Don't

```jsx
cache.getOrInsert(id, load(id));
groups.getOrInsert(name, createGroup());
```

#### Do

```jsx
cache.getOrInsertComputed(id, load);
groups.getOrInsert(name, []);
```

#### Fixed

```jsx
cache.getOrInsertComputed(id, load);
groups.getOrInsertComputed(name, () => createGroup());
```


### TypeScript

#### Don't

```ts
cache.getOrInsert(id, load(id));
groups.getOrInsert(name, createGroup());
```

#### Do

```ts
cache.getOrInsertComputed(id, load);
groups.getOrInsert(name, []);
```

#### Fixed

```ts
cache.getOrInsertComputed(id, load);
groups.getOrInsertComputed(name, () => createGroup());
```


### TSX

#### Don't

```tsx
cache.getOrInsert(id, load(id));
groups.getOrInsert(name, createGroup());
```

#### Do

```tsx
cache.getOrInsertComputed(id, load);
groups.getOrInsert(name, []);
```

#### Fixed

```tsx
cache.getOrInsertComputed(id, load);
groups.getOrInsertComputed(name, () => createGroup());
```
