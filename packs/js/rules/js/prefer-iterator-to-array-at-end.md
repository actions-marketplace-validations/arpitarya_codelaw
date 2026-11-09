---
id: js/prefer-iterator-to-array-at-end
title: "`.toArray()` last in an iterator helper chain"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the callback then runs lazily, one element at a time, instead of after the iterator is exhausted, and receives no `array` argument; the author accepts the change"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterator-to-array-at-end]
since: 0.5.0
---

`iterator.toArray().map(cb)` (and `.filter()`, `.flatMap()`) is reported at `toArray` when the callback is written inline, takes at most two parameters (no rest, no `arguments`) and so cannot read the `array` argument an `Array` method adds. The suggestion moves it: `iterator.map(cb).toArray()`. A named callback is opaque and not reported; a receiver known not to be an iterator (a literal, a class instance of a class declared in the file, a `const` bound to one) is not reported; a comment inside leaves the report without a suggestion.

**Why:** Iterator helpers are lazy: calling `.toArray()` first builds a temporary array and moves the rest of the chain onto `Array`.

```codelaw
native: js/prefer-iterator-to-array-at-end
```

## Examples

### JavaScript

#### Don't

```js
const doubled = iterator.toArray().map((x) => x * 2);
```

#### Do

```js
const doubled = iterator.map((x) => x * 2).toArray();
```

#### Fixed

```js
const doubled = iterator.map((x) => x * 2).toArray();
```


### JSX

#### Don't

```jsx
const doubled = iterator.toArray().map((x) => x * 2);
```

#### Do

```jsx
const doubled = iterator.map((x) => x * 2).toArray();
```

#### Fixed

```jsx
const doubled = iterator.map((x) => x * 2).toArray();
```


### TypeScript

#### Don't

```ts
const doubled = iterator.toArray().map((x) => x * 2);
```

#### Do

```ts
const doubled = iterator.map((x) => x * 2).toArray();
```

#### Fixed

```ts
const doubled = iterator.map((x) => x * 2).toArray();
```


### TSX

#### Don't

```tsx
const doubled = iterator.toArray().map((x) => x * 2);
```

#### Do

```tsx
const doubled = iterator.map((x) => x * 2).toArray();
```

#### Fixed

```tsx
const doubled = iterator.map((x) => x * 2).toArray();
```
