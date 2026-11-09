---
id: js/prefer-iterator-concat
title: "`Iterator.concat()` instead of a temporary array of spreads"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the fix is made where the array is passed to a constructor or function that only stores the values (`new Map/Set/WeakMap/WeakSet(…)`, a typed array, `Array.from`, `Object.fromEntries`) and no later spread is anything but a name or a literal; `Iterator.concat()` reads its iterables lazily and evaluates every argument first, so the other contexts are suggestions; a comment inside leaves the report without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterator-concat]
since: 0.5.0
---

An array of two or more spreads and nothing else (`[...a, ...b]`) passed where any iterable is accepted — `for…of`, `yield*`, `new Set/Map/WeakMap/WeakSet(…)`, a typed-array constructor, `Array.from`, `Object.fromEntries`, `Promise.all…` — is reported at the array: `Iterator.concat(a, b)` joins them without the temporary array. A mixed array (`[first, ...rest]`) is not reported, nor an array with a spread of a string (spreading one works, `Iterator.concat` of one throws), a spread of a `.toArray()` call, or `new Set([...a, ...b])` of values known to be sets.

**Why:** The temporary array holds every element of every part only to be walked once.

```codelaw
native: js/prefer-iterator-concat
```

## Examples

### JavaScript

#### Don't

```js
const all = new Set([...first, ...second]);
```

#### Do

```js
const all = new Set(Iterator.concat(first, second));
```

#### Fixed

```js
const all = new Set(Iterator.concat(first, second));
```


### JSX

#### Don't

```jsx
const all = new Set([...first, ...second]);
```

#### Do

```jsx
const all = new Set(Iterator.concat(first, second));
```

#### Fixed

```jsx
const all = new Set(Iterator.concat(first, second));
```


### TypeScript

#### Don't

```ts
const all = new Set([...first, ...second]);
```

#### Do

```ts
const all = new Set(Iterator.concat(first, second));
```

#### Fixed

```ts
const all = new Set(Iterator.concat(first, second));
```


### TSX

#### Don't

```tsx
const all = new Set([...first, ...second]);
```

#### Do

```tsx
const all = new Set(Iterator.concat(first, second));
```

#### Fixed

```tsx
const all = new Set(Iterator.concat(first, second));
```
