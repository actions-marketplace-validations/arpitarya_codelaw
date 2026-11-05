---
id: js/generator-without-yield
title: "A generator yields"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to yield or to drop the `*` is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/generator-without-yield]
since: 0.5.0
---

A generator function with a body contains a `yield`.

**Why:** A generator without `yield` returns an iterator over nothing, which is rarely meant.

```codelaw

match:
  any: [{kind: generator_function_declaration}, {kind: generator_function}, {kind: method_definition, regex: '^(static\s+)?(async\s+)?\*'}]
  has: { field: body, regex: '[^\s{}]' }
  not: { has: { stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: generator_function_declaration}, {kind: generator_function}] }, kind: yield_expression } }
message: "This generator never yields; make it a plain function"
```

## How to fix

1. Yield the values, or remove the `*` and return them.

## Examples

### JavaScript

#### Don't

```js
function* ids() {
  return [1, 2];
}
```

#### Do

```js
function* ids() {
  yield 1;
  yield 2;
}
```

### JSX

#### Don't

```jsx
function* ids() {
  return [1, 2];
}
```

#### Do

```jsx
function* ids() {
  yield 1;
  yield 2;
}
```

### TypeScript

#### Don't

```ts
function* ids() {
  return [1, 2];
}
```

#### Do

```ts
function* ids() {
  yield 1;
  yield 2;
}
```

### TSX

#### Don't

```tsx
function* ids() {
  return [1, 2];
}
```

#### Do

```tsx
function* ids() {
  yield 1;
  yield 2;
}
```
