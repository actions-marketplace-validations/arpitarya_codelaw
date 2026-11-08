---
id: js/no-unused-collection
title: "No collection that is filled but never read"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the collection is dead or its reader is missing is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-unused-collection]
since: 0.5.0
---

A local array, `Map`, `Set`, `WeakMap` or `WeakSet` whose every use is a statement that changes it — `push`, `add`, `set`, `delete`, `clear`, `sort`, … or an index assignment — and that is never read, returned, passed or iterated, is reported at its declaration. An exported variable is left alone.

**Why:** Work spent filling a collection nobody reads is wasted, and often a sign that the read was forgotten.

```codelaw
native: js/no-unused-collection
```

## How to fix

1. Remove the collection and the statements that fill it, or use what it collects.

## Examples

### JavaScript

#### Don't

```js
function track() {
  const seen = new Set();
  seen.add(1);
}
```

#### Do

```js
function track() {
  const seen = new Set();
  seen.add(1);
  return seen;
}
```


### JSX

#### Don't

```jsx
function track() {
  const seen = new Set();
  seen.add(1);
}
```

#### Do

```jsx
function track() {
  const seen = new Set();
  seen.add(1);
  return seen;
}
```


### TypeScript

#### Don't

```ts
function track() {
  const seen = new Set();
  seen.add(1);
}
```

#### Do

```ts
function track() {
  const seen = new Set();
  seen.add(1);
  return seen;
}
```


### TSX

#### Don't

```tsx
function track() {
  const seen = new Set();
  seen.add(1);
}
```

#### Do

```tsx
function track() {
  const seen = new Set();
  seen.add(1);
  return seen;
}
```
