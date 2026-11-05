---
id: js/arguments-usage
title: "Rest parameters rather than arguments"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the rest parameter needs a name and its uses change"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/arguments-usage]
since: 0.5.0
---

`arguments` is not read inside a function.

**Why:** A rest parameter is a real array, is named, and works in arrow functions.

```codelaw

match:
  kind: identifier
  regex: '^arguments$'
  inside: { stopBy: end, any: [{kind: function_declaration}, {kind: function_expression}, {kind: method_definition}] }
  not: { inside: { kind: member_expression, field: property } }
message: "Use a rest parameter (`...args`) instead of `arguments`"
```

## How to fix

1. Add `...args` to the parameter list and use `args` instead of `arguments`.

## Examples

### JavaScript

#### Don't

```js
function sum() {
  return [].reduce.call(arguments, (a, b) => a + b, 0);
}
```

#### Do

```js
function sum(...nums) {
  return nums.reduce((a, b) => a + b, 0);
}
```

### JSX

#### Don't

```jsx
function sum() {
  return [].reduce.call(arguments, (a, b) => a + b, 0);
}
```

#### Do

```jsx
function sum(...nums) {
  return nums.reduce((a, b) => a + b, 0);
}
```

### TypeScript

#### Don't

```ts
function sum() {
  return [].reduce.call(arguments, (a, b) => a + b, 0);
}
```

#### Do

```ts
function sum(...nums) {
  return nums.reduce((a, b) => a + b, 0);
}
```

### TSX

#### Don't

```tsx
function sum() {
  return [].reduce.call(arguments, (a, b) => a + b, 0);
}
```

#### Do

```tsx
function sum(...nums) {
  return nums.reduce((a, b) => a + b, 0);
}
```
