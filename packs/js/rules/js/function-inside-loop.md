---
id: js/function-inside-loop
title: "No function made in a loop that reads a variable the loop changes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "defining the function outside the loop, or giving each pass its own binding, restructures the loop"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/function-inside-loop]
since: 0.5.0
---

A function or arrow inside a loop (at any depth) is reported at its head when it reads a `var`, parameter or function name declared outside it that is written in the loop's header (its condition or update, or a `for…in`/`for…of` head) or directly in the loop's body. `let` and `const` are safe, as are an immediately invoked function and a callback passed to `forEach`, `map`, `filter`, `reduce`, `some`, `every`, `find`, `findIndex`, `sort`, `replace` or `each`. ESLint's `js/no-loop-func` asks the same with ESLint's own test.

**Why:** All the functions share the one variable, so when they run they see its last value.

```codelaw
native: js/function-inside-loop
```

## How to fix

1. Declare the loop variable with `let`, or move the function out of the loop and pass the value in.

## Examples

### JavaScript

#### Don't

```js
for (var k = 0; k < n; k++) {
  handlers.push(function () {
    return k;
  });
}
```

#### Do

```js
for (let k = 0; k < n; k++) {
  handlers.push(() => k);
}
```


### JSX

#### Don't

```jsx
for (var k = 0; k < n; k++) {
  handlers.push(function () {
    return k;
  });
}
```

#### Do

```jsx
for (let k = 0; k < n; k++) {
  handlers.push(() => k);
}
```


### TypeScript

#### Don't

```ts
for (var k = 0; k < n; k++) {
  handlers.push(function () {
    return k;
  });
}
```

#### Do

```ts
for (let k = 0; k < n; k++) {
  handlers.push(() => k);
}
```


### TSX

#### Don't

```tsx
for (var k = 0; k < n; k++) {
  handlers.push(function () {
    return k;
  });
}
```

#### Do

```tsx
for (let k = 0; k < n; k++) {
  handlers.push(() => k);
}
```
