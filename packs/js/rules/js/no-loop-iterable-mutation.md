---
id: js/no-loop-iterable-mutation
title: "No changing a collection while a `for…of` loop iterates it"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to iterate a copy or apply the changes after the loop is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-loop-iterable-mutation]
since: 0.5.0
---

Inside `for (… of x)` (or `x.keys()`, `x.values()`, `x.entries()`), a call that changes `x` — `push`, `splice`, `sort`, … on an array; `add`, `delete`, `clear` on a `Set`; `set`, `delete`, `clear` on a `Map`; any of them when the kind is unknown — is reported at the method. With a `const` loop binding, deleting the current item, and adding it back when it was not deleted first, are allowed. A copy (`[...x]`, `Array.from(x)`) is not live, so iterating one is fine.

**Why:** Adding while iterating can loop forever, and removing skips items, because the iterator sees the change.

```codelaw
native: js/no-loop-iterable-mutation
```

## How to fix

1. Iterate a copy: `for (const v of [...x])`.
2. Or collect the changes in the loop and apply them after it.

## Examples

### JavaScript

#### Don't

```js
const queue = [1, 2];
for (const item of queue) {
  queue.push(item * 2);
}
```

#### Do

```js
const queue = [1, 2];
for (const item of [...queue]) {
  queue.push(item * 2);
}
```


### JSX

#### Don't

```jsx
const queue = [1, 2];
for (const item of queue) {
  queue.push(item * 2);
}
```

#### Do

```jsx
const queue = [1, 2];
for (const item of [...queue]) {
  queue.push(item * 2);
}
```


### TypeScript

#### Don't

```ts
const queue = [1, 2];
for (const item of queue) {
  queue.push(item * 2);
}
```

#### Do

```ts
const queue = [1, 2];
for (const item of [...queue]) {
  queue.push(item * 2);
}
```


### TSX

#### Don't

```tsx
const queue = [1, 2];
for (const item of queue) {
  queue.push(item * 2);
}
```

#### Do

```tsx
const queue = [1, 2];
for (const item of [...queue]) {
  queue.push(item * 2);
}
```
