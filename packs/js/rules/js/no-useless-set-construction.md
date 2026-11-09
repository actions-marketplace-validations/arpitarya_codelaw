---
id: js/no-useless-set-construction
title: "No `new Set()` around a collection a `Set` method already accepts"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the fix is made where the replacement is a plain name or call and no comment is inside; otherwise the construction is reported without one"
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-set-construction]
since: 0.5.0
---

The set-algebra methods (`union`, `intersection`, `difference`, `symmetricDifference`, `isSubsetOf`, `isSupersetOf`, `isDisjointFrom`) take any set-like object (`size`, `has()`, `keys()`) and never change their receiver. On a value known to be a built-in `Set` (a `new Set()`, an annotation, a `const` alias), `new Set(…)` is reported when it wraps an argument that is already a `Set` or the keys of a `Map` (`new Set(map.keys())`, `new Set(set.values())`), when it copies the receiver (`new Set(a).union(b)`, if `b` is a known `Map` or `Set` with nothing in it that runs code), or when it copies the result of one of the four set-returning methods. `new Set(map.values())`, `new Set(map.entries())` and `new Set(map)` are not reported: those are not the keys. Optional calls, spreads, explicit type arguments and TypeScript wrappers are left alone.

**Why:** Each extra `Set` copies every element for nothing; the method already reads the collection it is given.

```codelaw
native: js/no-useless-set-construction
```

## Examples

### JavaScript

#### Don't

```js
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(new Set(b));
```

#### Do

```js
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```

#### Fixed

```js
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```


### JSX

#### Don't

```jsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(new Set(b));
```

#### Do

```jsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```

#### Fixed

```jsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```


### TypeScript

#### Don't

```ts
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(new Set(b));
```

#### Do

```ts
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```

#### Fixed

```ts
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```


### TSX

#### Don't

```tsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(new Set(b));
```

#### Do

```tsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```

#### Fixed

```tsx
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const both = a.union(b);
```
