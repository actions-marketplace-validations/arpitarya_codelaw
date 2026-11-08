---
id: js/no-use-of-empty-return-value
title: "No use of the result of a function that returns nothing"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "either the function should return a value or the caller should not use one, the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-use-of-empty-return-value]
since: 0.5.0
---

A call of a function the file declares once (a declaration or a variable bound to a function), which never returns a value, is reported when its result is used: assigned, passed, compared, tested. A call as a statement, after `await`, `return` or `throw`, under a unary operator, on the right of `&&`/`||`/`??` or in a branch of `?:` is not a use. Async functions, generators, expression-bodied arrows and empty stubs are left alone.

**Why:** The result is always `undefined`, so the code that uses it can't be doing what it intends.

```codelaw
native: js/no-use-of-empty-return-value
```

## How to fix

1. Return the value from the function, or stop using the call's result.

## Examples

### JavaScript

#### Don't

```js
function log(message) {
  out.write(message);
}
const written = log("x");
```

#### Do

```js
function log(message) {
  out.write(message);
}
log("x");
```


### JSX

#### Don't

```jsx
function log(message) {
  out.write(message);
}
const written = log("x");
```

#### Do

```jsx
function log(message) {
  out.write(message);
}
log("x");
```


### TypeScript

#### Don't

```ts
function log(message) {
  out.write(message);
}
const written = log("x");
```

#### Do

```ts
function log(message) {
  out.write(message);
}
log("x");
```


### TSX

#### Don't

```tsx
function log(message) {
  out.write(message);
}
const written = log("x");
```

#### Do

```tsx
function log(message) {
  out.write(message);
}
log("x");
```
