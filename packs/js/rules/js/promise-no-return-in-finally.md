---
id: js/promise-no-return-in-finally
title: "A finally callback returns nothing"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the returned value is dropped from the result either way; the reviewer confirms it should not move to a `.then()`"
stability: preview
ported_from: [eslint-plugin-promise/no-return-in-finally]
since: 0.5.0
---

The callback passed to `.finally()` does not `return` a value.

**Why:** Whatever `.finally()`'s callback returns is thrown away — the chain keeps the value before it — so a `return x` there reads as if it changed the result and never does.

```codelaw
match:
  kind: return_statement
  regex: '^return\s*[^\s;]'
  has: { pattern: $E }
  inside:
    any: [{kind: arrow_function}, {kind: function_expression}]
    stopBy:
      any: [{kind: arrow_function}, {kind: function_expression}, {kind: function_declaration}, {kind: method_definition}]
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: '^finally$' } }
message: "A value returned from `.finally()` is ignored; move it to a `.then()` or drop it"
fix: "$E;"
```

## How to fix

1. Move the value into a `.then()` before the `.finally()`, or delete the `return` value if it was not meant to change the result.

## Examples

### JavaScript

#### Don't

```js
load().finally(() => {
  return cleanup();
});
```

#### Fixed

```js
load().finally(() => {
  cleanup();
});
```

#### Do

```js
load().finally(() => {
  cleanup();
});
```

### JSX

#### Don't

```jsx
load().finally(() => {
  return cleanup();
});
```

#### Fixed

```jsx
load().finally(() => {
  cleanup();
});
```

#### Do

```jsx
load().finally(() => {
  cleanup();
});
```

### TypeScript

#### Don't

```ts
load().finally(() => {
  return cleanup();
});
```

#### Fixed

```ts
load().finally(() => {
  cleanup();
});
```

#### Do

```ts
load().finally(() => {
  cleanup();
});
```

### TSX

#### Don't

```tsx
load().finally(() => {
  return cleanup();
});
```

#### Fixed

```tsx
load().finally(() => {
  cleanup();
});
```

#### Do

```tsx
load().finally(() => {
  cleanup();
});
```
