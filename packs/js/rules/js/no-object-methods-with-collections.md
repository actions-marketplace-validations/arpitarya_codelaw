---
id: js/no-object-methods-with-collections
title: "No `Object.keys()`, `values()` or `entries()` on a `Map` or `Set`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Array.from(map.keys())` returns the collection's keys, where `Object.keys(map)` returned nothing; the author accepts the new result"
stability: preview
ported_from: [eslint-plugin-unicorn/no-object-methods-with-collections]
since: 0.5.0
---

`Object.keys(x)`, `Object.values(x)` and `Object.entries(x)` with one argument known to be a built-in `Map` or `Set` (not `WeakMap`/`WeakSet`) is reported at the method name: their contents are not own enumerable properties, so the result is almost always `[]`. The suggestion is `Array.from(x.keys())` (the same method); it is withheld when a comment is inside the call. A collection the engine cannot see (a value from a call, an imported type) is not reported.

**Why:** A `Map` or `Set` keeps its contents out of its own properties, so the `Object` methods see an empty object.

```codelaw
native: js/no-object-methods-with-collections
```

## Examples

### JavaScript

#### Don't

```js
const map = new Map([['a', 1]]);
const keys = Object.keys(map);
```

#### Do

```js
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```

#### Fixed

```js
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```


### JSX

#### Don't

```jsx
const map = new Map([['a', 1]]);
const keys = Object.keys(map);
```

#### Do

```jsx
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```

#### Fixed

```jsx
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```


### TypeScript

#### Don't

```ts
const map = new Map([['a', 1]]);
const keys = Object.keys(map);
```

#### Do

```ts
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```

#### Fixed

```ts
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```


### TSX

#### Don't

```tsx
const map = new Map([['a', 1]]);
const keys = Object.keys(map);
```

#### Do

```tsx
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```

#### Fixed

```tsx
const map = new Map([['a', 1]]);
const keys = Array.from(map.keys());
```
