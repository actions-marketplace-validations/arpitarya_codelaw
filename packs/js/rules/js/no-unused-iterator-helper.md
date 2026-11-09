---
id: js/no-unused-iterator-helper
title: "No lazy iterator helper whose result is thrown away"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`.forEach()` runs the callback now and reads the iterator, which the lazy call never did; the author accepts that"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unused-iterator-helper]
since: 0.5.0
---

`.map()`, `.filter()`, `.flatMap()`, `.take()` and `.drop()` on an iterator return another iterator and read nothing until it is read. A call whose result is discarded — as a statement, under `void`, under `await`, in a `for` initializer or update — never runs its callback. The receiver must be known to be an iterator: `.values()`, `.keys()`, `.entries()` or `.matchAll()`, an `Iterator.from/concat/zip` call, a helper chain on one, a call of a local synchronous generator, or a plain `const` of one. A discarded `.map(callback)` gets the suggestion `.forEach(callback)` (one argument, no type arguments, no TypeScript assertion around it). A receiver from a call the engine cannot see, an object method or an async generator is not known and is not reported.

**Why:** An iterator helper is a recipe, not an action: dropping its result drops the work, and the code reads as if it had run.

```codelaw
native: js/no-unused-iterator-helper
```

## Examples

### JavaScript

#### Don't

```js
items.values().map(save);
```

#### Do

```js
for (const item of items.values()) {
  save(item);
}
```

#### Fixed

```js
items.values().forEach(save);
```


### JSX

#### Don't

```jsx
items.values().map(save);
```

#### Do

```jsx
for (const item of items.values()) {
  save(item);
}
```

#### Fixed

```jsx
items.values().forEach(save);
```


### TypeScript

#### Don't

```ts
items.values().map(save);
```

#### Do

```ts
for (const item of items.values()) {
  save(item);
}
```

#### Fixed

```ts
items.values().forEach(save);
```


### TSX

#### Don't

```tsx
items.values().map(save);
```

#### Do

```tsx
for (const item of items.values()) {
  save(item);
}
```

#### Fixed

```tsx
items.values().forEach(save);
```
