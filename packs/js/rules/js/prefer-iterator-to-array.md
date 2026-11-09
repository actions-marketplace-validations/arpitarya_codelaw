---
id: js/prefer-iterator-to-array
title: "`.toArray()` instead of a temporary array spread of one iterator"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the fix is made where the spread is of an `Iterator.from()`/`Iterator.concat()` call or a helper chain on one; `.values()`, `.keys()`, `.entries()` and `.matchAll()` may be a custom method that does not return an iterator, so those are suggestions; a comment inside leaves the report without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterator-to-array]
since: 0.5.0
---

`[...iterator]` — an array literal of exactly one spread — is reported when the spread value is `Iterator.from(x)`, `Iterator.concat(…)`, a helper chain (`.map(f)`, `.filter(f)`, `.flatMap(f)`, `.take(n)`, `.drop(n)`) on one of those, or the result of `.values()`, `.keys()`, `.entries()` or `.matchAll(re)`. Where any iterable is accepted anyway (`new Set([...map.values()])`, `for…of`, `yield*`, a spread) the array is not reported. A set of several spreads, or a spread with other elements, is left alone, as is a plain `[...set]`: not every iterable has `.toArray()`.

**Why:** `.toArray()` states the conversion and keeps a helper chain reading left to right.

```codelaw
native: js/prefer-iterator-to-array
```

## Examples

### JavaScript

#### Don't

```js
const values = [...Iterator.from(source)];
```

#### Do

```js
const values = Iterator.from(source).toArray();
```

#### Fixed

```js
const values = Iterator.from(source).toArray();
```


### JSX

#### Don't

```jsx
const values = [...Iterator.from(source)];
```

#### Do

```jsx
const values = Iterator.from(source).toArray();
```

#### Fixed

```jsx
const values = Iterator.from(source).toArray();
```


### TypeScript

#### Don't

```ts
const values = [...Iterator.from(source)];
```

#### Do

```ts
const values = Iterator.from(source).toArray();
```

#### Fixed

```ts
const values = Iterator.from(source).toArray();
```


### TSX

#### Don't

```tsx
const values = [...Iterator.from(source)];
```

#### Do

```tsx
const values = Iterator.from(source).toArray();
```

#### Fixed

```tsx
const values = Iterator.from(source).toArray();
```
