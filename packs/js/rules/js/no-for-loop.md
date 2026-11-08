---
id: js/no-for-loop
title: "`for…of` instead of an index loop over an array"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: rewritten when the update is `i++`, `++i` or `i += 1`, the index and length are not used after the loop, and nothing in the loop calls or deletes `arr[i]`; otherwise reported without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/no-for-loop]
since: 0.5.0
---

`for (let i = 0; i < arr.length; i++)` (or with the length cached, `i < len`) whose body only reads `arr[i]` and never changes `i` is reported: it is a `for…of`. The fix uses the body's `const x = arr[i]` as the loop variable when it starts the body, or the array's singular name; when `i` is used for more than indexing it writes `for (const [i, x] of arr.entries())`. A write to `arr[i]`, any other use of the array, or a different step leaves the loop alone.

**Why:** `for…of` says the loop visits each item, with no index to get wrong.

```codelaw
native: js/no-for-loop
```

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i < items.length; i++) {
  const item = items[i];
  log(item);
}
```

#### Do

```js
for (const item of items) {
  log(item);
}
```

#### Fixed

```js
for (const item of items) {
  log(item);
}
```


### JSX

#### Don't

```jsx
for (let i = 0; i < items.length; i++) {
  const item = items[i];
  log(item);
}
```

#### Do

```jsx
for (const item of items) {
  log(item);
}
```

#### Fixed

```jsx
for (const item of items) {
  log(item);
}
```


### TypeScript

#### Don't

```ts
for (let i = 0; i < items.length; i++) {
  const item = items[i];
  log(item);
}
```

#### Do

```ts
for (const item of items) {
  log(item);
}
```

#### Fixed

```ts
for (const item of items) {
  log(item);
}
```


### TSX

#### Don't

```tsx
for (let i = 0; i < items.length; i++) {
  const item = items[i];
  log(item);
}
```

#### Do

```tsx
for (const item of items) {
  log(item);
}
```

#### Fixed

```tsx
for (const item of items) {
  log(item);
}
```
