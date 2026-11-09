---
id: js/no-useless-iterator-to-array
title: "No `.toArray()` where an iterable or an iterator method does"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "`.toArray()` is removed where the context reads the whole iterable at once (a collection or typed-array constructor, `Array.from`, `Object.fromEntries`, a spread) and no comment is inside; where removing it changes when the iterator is read (`for…of`, `yield*`, `Promise.all…`, `every()`/`find()`/`forEach()`/`some()`/`reduce()`) it is a suggestion"
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-iterator-to-array]
since: 0.5.0
---

`.toArray()` is reported at its name when the context accepts an iterable: `new Map/WeakMap/Set/WeakSet(…)` or a typed-array constructor, `Array.from(…)` and `TypedArray.from(…)`, `Object.fromEntries(…)`, `Promise.all/allSettled/any/race(…)`, `for…of`, `yield*`, a spread in an array or an argument list; and before `.every()`, `.find()`, `.forEach()`, `.some()` or `.reduce()`, which `Iterator` has too — unless the callback is not written inline or can read the `array` argument (three parameters, or four for `reduce`; a rest parameter; `arguments`). `.filter()`, `.map()` and `.flatMap()` are not reported: their `Iterator` versions return iterators. `toArray()` is a common method name, so a receiver known not to be an iterator — a literal, a function, a class or an instance of a class declared in the file, or a `const` bound to one or to a call of `toArray`, `slice`, `concat` or `split` — is skipped; any other receiver is taken for an iterator, as upstream does. With a second argument (`Array.from(it.toArray(), map)`) the removal is a suggestion.

**Why:** `.toArray()` builds an array only for the next step to walk it again.

```codelaw
native: js/no-useless-iterator-to-array
```

## Examples

### JavaScript

#### Don't

```js
const set = new Set(iterator.toArray());
```

#### Do

```js
const set = new Set(iterator);
```

#### Fixed

```js
const set = new Set(iterator);
```


### JSX

#### Don't

```jsx
const set = new Set(iterator.toArray());
```

#### Do

```jsx
const set = new Set(iterator);
```

#### Fixed

```jsx
const set = new Set(iterator);
```


### TypeScript

#### Don't

```ts
const set = new Set(iterator.toArray());
```

#### Do

```ts
const set = new Set(iterator);
```

#### Fixed

```ts
const set = new Set(iterator);
```


### TSX

#### Don't

```tsx
const set = new Set(iterator.toArray());
```

#### Do

```tsx
const set = new Set(iterator);
```

#### Fixed

```tsx
const set = new Set(iterator);
```
