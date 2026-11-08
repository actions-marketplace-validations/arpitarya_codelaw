---
id: js/consistent-function-scoping
title: "No inner function that uses nothing from its outer function"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the function changes where it is defined and what it is named next to, the author's layout"
fix_exception: restructure
options: { checkArrowFunctions: { type: "boolean", default: true, description: "check arrow functions as well as function declarations and expressions" } }
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-function-scoping]
since: 0.5.0
---

A function declared inside another function — a declaration, a `const f = () => …` or a returned function — that refers to no variable, parameter or function of the enclosing function (or the blocks between) is reported at its head: it can live one level up. Callbacks passed as arguments, object values, functions in an immediately invoked function, in a React hook's callback, in a `jest.mock`/`vi.mock` factory, and arrows that use the outer `this` are left alone.

**Why:** A function recreated on every call of its parent costs allocations and hides that it doesn't depend on the parent.

```codelaw
native: js/consistent-function-scoping
```

## How to fix

1. Move the function to the enclosing scope (often the module), and keep its name.

## Examples

### JavaScript

#### Don't

```js
function format(items) {
  function pad(text) {
    return text.padStart(4);
  }
  return items.map(pad);
}
```

#### Do

```js
function pad(text) {
  return text.padStart(4);
}
function format(items) {
  return items.map(pad);
}
```


### JSX

#### Don't

```jsx
function format(items) {
  function pad(text) {
    return text.padStart(4);
  }
  return items.map(pad);
}
```

#### Do

```jsx
function pad(text) {
  return text.padStart(4);
}
function format(items) {
  return items.map(pad);
}
```


### TypeScript

#### Don't

```ts
function format(items) {
  function pad(text) {
    return text.padStart(4);
  }
  return items.map(pad);
}
```

#### Do

```ts
function pad(text) {
  return text.padStart(4);
}
function format(items) {
  return items.map(pad);
}
```


### TSX

#### Don't

```tsx
function format(items) {
  function pad(text) {
    return text.padStart(4);
  }
  return items.map(pad);
}
```

#### Do

```tsx
function pad(text) {
  return text.padStart(4);
}
function format(items) {
  return items.map(pad);
}
```
