---
id: js/no-empty-collection
title: "No reading a collection nothing ever adds to"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "either the code that fills the collection is missing or the reads are dead, which the author decides"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-empty-collection]
since: 0.5.0
---

A variable given an empty array, `Map`, `Set`, `WeakMap` or `WeakSet` (in its declaration or by `x = [];`), whose every other use only reads it — `forEach`, `map`, `includes`, `get`, `has`, `join`, … , `for…of`, an index read — is reported at each read: it is always empty. Any other use (an add, being passed or returned, an index write, any other value) leaves it alone.

**Why:** Reading a collection that is never filled does nothing, which usually means the code that fills it was lost.

```codelaw
native: js/no-empty-collection
```

## How to fix

1. Fill the collection where it is meant to be filled, or remove it and the reads.

## Examples

### JavaScript

#### Don't

```js
function run() {
  const queue = [];
  queue.forEach(start);
}
```

#### Do

```js
function run(tasks) {
  const queue = [...tasks];
  queue.forEach(start);
}
```


### JSX

#### Don't

```jsx
function run() {
  const queue = [];
  queue.forEach(start);
}
```

#### Do

```jsx
function run(tasks) {
  const queue = [...tasks];
  queue.forEach(start);
}
```


### TypeScript

#### Don't

```ts
function run() {
  const queue = [];
  queue.forEach(start);
}
```

#### Do

```ts
function run(tasks) {
  const queue = [...tasks];
  queue.forEach(start);
}
```


### TSX

#### Don't

```tsx
function run() {
  const queue = [];
  queue.forEach(start);
}
```

#### Do

```tsx
function run(tasks) {
  const queue = [...tasks];
  queue.forEach(start);
}
```
