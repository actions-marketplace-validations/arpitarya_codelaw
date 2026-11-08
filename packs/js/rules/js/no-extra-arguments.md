---
id: js/no-extra-arguments
title: "No call with more arguments than the function takes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to drop the extra arguments or add parameters is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-extra-arguments]
since: 0.5.0
---

A call of a function the file declares once — a declaration, a variable bound to a function, or an immediately invoked function — with more arguments than it has parameters is reported at the callee. A rest parameter, a spread argument, a function that reads `arguments`, and an empty function with no parameters (a stub) are left alone; methods and parameters are not judged.

**Why:** The extra values are silently ignored, which usually means a parameter is missing or the wrong function is called.

```codelaw
native: js/no-extra-arguments
```

## How to fix

1. Remove the extra arguments, or add the parameters the function should take.

## Examples

### JavaScript

#### Don't

```js
function add(a, b) {
  return a + b;
}
add(1, 2, 3);
```

#### Do

```js
function add(a, b) {
  return a + b;
}
add(1, 2);
```


### JSX

#### Don't

```jsx
function add(a, b) {
  return a + b;
}
add(1, 2, 3);
```

#### Do

```jsx
function add(a, b) {
  return a + b;
}
add(1, 2);
```


### TypeScript

#### Don't

```ts
function add(a, b) {
  return a + b;
}
add(1, 2, 3);
```

#### Do

```ts
function add(a, b) {
  return a + b;
}
add(1, 2);
```


### TSX

#### Don't

```tsx
function add(a, b) {
  return a + b;
}
add(1, 2, 3);
```

#### Do

```tsx
function add(a, b) {
  return a + b;
}
add(1, 2);
```
