---
id: js/no-identical-functions
title: "No two functions with the same body"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which function stays, and how the other calls it, is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-identical-functions]
since: 0.5.0
---

A function whose parameters and body (of at least three lines) are the same as an earlier function's in the file, ignoring whitespace, is reported: call or reuse the first one.

**Why:** Duplicated code is fixed in one copy and left broken in the other.

```codelaw
native: js/no-identical-functions
```

## How to fix

1. Keep one function and call it, or assign it, where the other was used.

## Examples

### JavaScript

#### Don't

```js
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
function cost(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
```

#### Do

```js
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
const cost = total;
```


### JSX

#### Don't

```jsx
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
function cost(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
```

#### Do

```jsx
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
const cost = total;
```


### TypeScript

#### Don't

```ts
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
function cost(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
```

#### Do

```ts
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
const cost = total;
```


### TSX

#### Don't

```tsx
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
function cost(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
```

#### Do

```tsx
function total(items) {
  let sum = 0;
  for (const i of items) sum += i.price;
  return sum;
}
const cost = total;
```
