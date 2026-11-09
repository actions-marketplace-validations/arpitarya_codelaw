---
id: js/prefer-iterator-helpers
title: "Iterator helpers before an iterator is made into an array"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`take()` closes the iterator early and callbacks run lazily, which skips side effects and errors the array version ran; the author accepts that"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterator-helpers]
since: 0.5.0
---

Two shapes are reported. **Terminal methods:** `[...iterator].find(cb)` or `Array.from(iterator).some(cb)` (also `every`, `forEach`, `reduce` with one or two arguments) — the suggestion calls the method on the iterator. **Slices:** `.toArray().slice(a, b)`, `[...iterator].slice(a, b)` or `Array.from(iterator).slice(a, b)` with nonnegative integer literal bounds — the suggestion is `iterator.drop(a).take(b - a).toArray()`; `.slice()` and `.slice(0)` alone, and bounds that are not literals, are not reported. A callback that can read the `array` argument (a third parameter, four for `reduce`, a rest parameter, `arguments`) or that the file does not declare is not reported. The iterator must be known: what `.values()`, `.keys()`, `.entries()` or `.matchAll()` returns, an `Iterator.from/concat/zip` call, a helper chain on one, a call of a local synchronous generator, a plain `const` of one, or a value annotated with an iterator type; a zero-argument `.toArray()` is itself the evidence for a slice. A comment inside leaves the report without a suggestion; calls with explicit type arguments are skipped.

**Why:** Materializing the iterator first stores every element, even the ones a `find()` or a `take()` never needs.

```codelaw
native: js/prefer-iterator-helpers
```

## Examples

### JavaScript

#### Don't

```js
const hit = [...map.values()].find((v) => v.id === id);
```

#### Do

```js
const hit = map.values().find((v) => v.id === id);
```

#### Fixed

```js
const hit = map.values().find((v) => v.id === id);
```


### JSX

#### Don't

```jsx
const hit = [...map.values()].find((v) => v.id === id);
```

#### Do

```jsx
const hit = map.values().find((v) => v.id === id);
```

#### Fixed

```jsx
const hit = map.values().find((v) => v.id === id);
```


### TypeScript

#### Don't

```ts
const hit = [...map.values()].find((v) => v.id === id);
```

#### Do

```ts
const hit = map.values().find((v) => v.id === id);
```

#### Fixed

```ts
const hit = map.values().find((v) => v.id === id);
```


### TSX

#### Don't

```tsx
const hit = [...map.values()].find((v) => v.id === id);
```

#### Do

```tsx
const hit = map.values().find((v) => v.id === id);
```

#### Fixed

```tsx
const hit = map.values().find((v) => v.id === id);
```
