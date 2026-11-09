---
id: js/prefer-array-iterable-methods
title: "Iterate an array, or use `keys()`, instead of `entries()` that drops half"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the fix is made unless a comment is inside the loop head or the kept binding has a default or a rest; otherwise the loop is reported without one"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-iterable-methods]
since: 0.5.0
---

`for (const [, value] of array.entries())` is reported at `entries` when the index is unused (iterate the array: `for (const value of array)`), and `for (const [index] of array.entries())` when the value is unused (`array.keys()`). Only `for…of` over a value known to be an array — an array literal, `Array()`, `Array.from()`/`of()`, `Object.keys()` and the like, `sort()`/`reverse()`/… of an array, a rest parameter, an annotation `T[]` or `Array<T>`, or a `const` alias — is checked; a `Map`'s `.entries()` has the same shape and must not be rewritten. A binding that is destructured or written to, and `for await`, are left alone.

**Why:** `entries()` builds a pair per element; when one half is thrown away, the loop says more than it means.

```codelaw
native: js/prefer-array-iterable-methods
```

## Examples

### JavaScript

#### Don't

```js
const list = [1, 2];
for (const [, value] of list.entries()) {
  use(value);
}
```

#### Do

```js
const list = [1, 2];
for (const value of list) {
  use(value);
}
```

#### Fixed

```js
const list = [1, 2];
for (const value of list) {
  use(value);
}
```


### JSX

#### Don't

```jsx
const list = [1, 2];
for (const [, value] of list.entries()) {
  use(value);
}
```

#### Do

```jsx
const list = [1, 2];
for (const value of list) {
  use(value);
}
```

#### Fixed

```jsx
const list = [1, 2];
for (const value of list) {
  use(value);
}
```


### TypeScript

#### Don't

```ts
const list = [1, 2];
for (const [, value] of list.entries()) {
  use(value);
}
```

#### Do

```ts
const list = [1, 2];
for (const value of list) {
  use(value);
}
```

#### Fixed

```ts
const list = [1, 2];
for (const value of list) {
  use(value);
}
```


### TSX

#### Don't

```tsx
const list = [1, 2];
for (const [, value] of list.entries()) {
  use(value);
}
```

#### Do

```tsx
const list = [1, 2];
for (const value of list) {
  use(value);
}
```

#### Fixed

```tsx
const list = [1, 2];
for (const value of list) {
  use(value);
}
```
