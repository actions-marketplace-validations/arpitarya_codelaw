---
id: js/no-duplicate-loops
title: "No `for…of` over a `.map()` or `.filter()`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "folding the callback into the loop body rewrites the loop"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-duplicate-loops]
since: 0.5.0
---

`for (const x of list.filter(f))` (or `.map(f)`) walks the array twice: once to build a new one, once in the loop. It is reported at the method. A receiver that is already a lazy iterator — `Iterator.from()`, `.values()`, `.keys()`, `.entries()`, `.matchAll()` and their `map`/`filter`/`take`/`drop` chains — is not an array, and is left alone.

**Why:** Doing the work in the loop body saves the intermediate array and a second pass.

```codelaw
native: js/no-duplicate-loops
```

## How to fix

1. Move the callback into the loop: `for (const x of list) { if (!f(x)) continue; … }`.

## Examples

### JavaScript

#### Don't

```js
for (const user of users.filter(isActive)) {
  notify(user);
}
```

#### Do

```js
for (const user of users) {
  if (isActive(user)) notify(user);
}
```


### JSX

#### Don't

```jsx
for (const user of users.filter(isActive)) {
  notify(user);
}
```

#### Do

```jsx
for (const user of users) {
  if (isActive(user)) notify(user);
}
```


### TypeScript

#### Don't

```ts
for (const user of users.filter(isActive)) {
  notify(user);
}
```

#### Do

```ts
for (const user of users) {
  if (isActive(user)) notify(user);
}
```


### TSX

#### Don't

```tsx
for (const user of users.filter(isActive)) {
  notify(user);
}
```

#### Do

```tsx
for (const user of users) {
  if (isActive(user)) notify(user);
}
```
