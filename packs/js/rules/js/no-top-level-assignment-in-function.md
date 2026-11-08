---
id: js/no-top-level-assignment-in-function
title: "No assigning a module-level variable inside a function"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the state should live instead (a return value, an object, a parameter) is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-top-level-assignment-in-function]
since: 0.5.0
---

A variable declared at the top of a module (an import included) is not assigned, incremented or destructured into from inside a function, arrow or method. Assignments at the top level, in a class's static block or field value, and changes to an object's properties are not reported.

**Why:** A function that rewrites module state hides a dependency between its callers and makes the module's behaviour depend on the order its functions are called in.

```codelaw
native: js/no-top-level-assignment-in-function
```

## How to fix

1. Return the new value from the function and assign it where it is called.
2. Or keep the state in an object (a class instance, a `Map`) that the functions receive.

## Examples

### JavaScript

#### Don't

```js
let count = 0;
export function increment() {
  count += 1;
}
```

#### Do

```js
export function increment(count) {
  return count + 1;
}
```


### JSX

#### Don't

```jsx
let count = 0;
export function increment() {
  count += 1;
}
```

#### Do

```jsx
export function increment(count) {
  return count + 1;
}
```


### TypeScript

#### Don't

```ts
let count = 0;
export function increment() {
  count += 1;
}
```

#### Do

```ts
export function increment(count) {
  return count + 1;
}
```


### TSX

#### Don't

```tsx
let count = 0;
export function increment() {
  count += 1;
}
```

#### Do

```tsx
export function increment(count) {
  return count + 1;
}
```
