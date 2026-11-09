---
id: js/no-collection-bracket-access
title: "No bracket access on a `Map`, `Set`, `WeakMap` or `WeakSet`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the write or delete moves from a plain property to the collection's entries, which code that read the property will no longer see; the author accepts that"
stability: preview
ported_from: [eslint-plugin-unicorn/no-collection-bracket-access]
since: 0.5.0
---

`map['key']` on a value known to be a built-in `Map`, `Set`, `WeakMap` or `WeakSet` — made by `new Map()` (or the others) with the global constructor, annotated `Map<K, V>` (or `ReadonlyMap`, `ReadonlySet`), or a `const` alias of one — reads, writes or deletes an ordinary property of the object, never an entry. A key that names a real member (`map['size']`, `map[Symbol.iterator]`, a `const` holding such a string) is allowed. Where the statement is a plain `map[k] = v` (`Map`, `WeakMap`) or `delete map[k]` on a name, `this` or a member, the suggestion is `map.set(k, v)` or `map.delete(k)`; reads and `Set` writes are reported without one. Types the engine cannot see (a value from a call, an imported type) are not known to be collections and are not reported; upstream also reads TypeScript's inferred types when a program is loaded, which this rule does not.

**Why:** Entries live in the collection, not in properties: a bracket write is lost to `has()`, `get()` and iteration, and a bracket read returns `undefined`.

```codelaw
native: js/no-collection-bracket-access
```

## Examples

### JavaScript

#### Don't

```js
const map = new Map();
map['foo'] = 'bar';
```

#### Do

```js
const map = new Map();
map.set('foo', 'bar');
```

#### Fixed

```js
const map = new Map();
map.set('foo', 'bar');
```


### JSX

#### Don't

```jsx
const map = new Map();
map['foo'] = 'bar';
```

#### Do

```jsx
const map = new Map();
map.set('foo', 'bar');
```

#### Fixed

```jsx
const map = new Map();
map.set('foo', 'bar');
```


### TypeScript

#### Don't

```ts
const map = new Map();
map['foo'] = 'bar';
```

#### Do

```ts
const map = new Map();
map.set('foo', 'bar');
```

#### Fixed

```ts
const map = new Map();
map.set('foo', 'bar');
```


### TSX

#### Don't

```tsx
const map = new Map();
map['foo'] = 'bar';
```

#### Do

```tsx
const map = new Map();
map.set('foo', 'bar');
```

#### Fixed

```tsx
const map = new Map();
map.set('foo', 'bar');
```
