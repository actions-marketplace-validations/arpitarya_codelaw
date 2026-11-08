---
id: js/no-loop-func
title: "No function made in a loop that reads a variable the loop changes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the function out of the loop, or binding the value per pass, changes the loop's structure"
fix_exception: restructure
stability: preview
ported_from: [eslint/no-loop-func]
since: 0.5.0
---

A function or arrow created in a loop body or test (an immediately invoked one aside) is reported when it refers to a variable declared outside it that the loop can change: a `var` (also the loop's own `var` counter), or a `let` declared outside the loop that is assigned in or after the loop or from another function. `const`s, functions, imports, and `let`s declared inside the loop (a fresh binding each pass) are safe. Undeclared globals are not judged.

**Why:** Every function made by the loop shares the one variable, so when they run they all see its last value.

```codelaw
native: js/no-loop-func
```

## How to fix

1. Declare the loop variable with `let` or `const` in the loop head or body, so each pass gets its own.
2. Or move the function out of the loop and pass the value in.

## Examples

### JavaScript

#### Don't

```js
for (var i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```

#### Do

```js
for (let i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```


### JSX

#### Don't

```jsx
for (var i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```

#### Do

```jsx
for (let i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```


### TypeScript

#### Don't

```ts
for (var i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```

#### Do

```ts
for (let i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```


### TSX

#### Don't

```tsx
for (var i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```

#### Do

```tsx
for (let i = 0; i < 3; i++) {
  buttons[i].onclick = () => show(i);
}
```
