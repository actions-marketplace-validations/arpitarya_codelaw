---
id: js/no-invalid-this
title: "No `this` where strict code makes it `undefined`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the object the code means has to be passed in, which changes the function's signature"
fix_exception: restructure
stability: preview
ported_from: [eslint/no-invalid-this]
since: 0.5.0
---

In strict code (modules, and `.cjs` files with `"use strict"`), `this` outside a class or object method is `undefined`. It is reported at the top level and in a plain function, unless the function is written to receive a `this`: a method, a class field or static block, a function with a capitalised name (a constructor), a property value or a function assigned to a property, one passed to `.call()`, `.apply()` or `.bind()` with an object, one given an object as `thisArg` (`arr.map(fn, obj)`, `Array.from(x, fn, obj)`, `Reflect.apply`), one with a TypeScript `this` parameter, or one documented `@this`. Arrow functions take the `this` around them.

**Why:** Reading a property of `undefined` throws at run time.

```codelaw
native: js/no-invalid-this
```

## How to fix

1. Pass the object as a parameter, or make the function a method.

## Examples

### JavaScript

#### Don't

```js
function reset() {
  this.count = 0;
}
```

#### Do

```js
function reset(counter) {
  counter.count = 0;
}
```


### JSX

#### Don't

```jsx
function reset() {
  this.count = 0;
}
```

#### Do

```jsx
function reset(counter) {
  counter.count = 0;
}
```


### TypeScript

#### Don't

```ts
function reset() {
  this.count = 0;
}
```

#### Do

```ts
function reset(counter) {
  counter.count = 0;
}
```


### TSX

#### Don't

```tsx
function reset() {
  this.count = 0;
}
```

#### Do

```tsx
function reset(counter) {
  counter.count = 0;
}
```
