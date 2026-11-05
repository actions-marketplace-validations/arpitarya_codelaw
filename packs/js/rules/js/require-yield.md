---
id: js/require-yield
title: "A generator function yields"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "dropping the `*` makes a plain function, right only if the generator was never meant to yield; the reviewer confirms, since adding the missing `yield` is the author's"
stability: preview
ported_from: [eslint/require-yield]
since: 0.1.0
---

A non-empty generator function contains at least one `yield`.

**Why:** A generator without `yield` is either unfinished or should be a plain function.

```codelaw
match:
  any:
    - kind: generator_function_declaration
    - kind: generator_function
  has: { pattern: $$STAR, regex: '^\*$' }
  not:
    any:
      - has: { field: body, regex: '^\{\s*\}$' }
      - has:
          kind: yield_expression
          stopBy:
            any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}]
message: "A generator that never yields is a plain function"
fix:
  - remove: $$STAR
```

## How to fix

1. Add the `yield` the generator was meant to have, or remove the `*`; the fix removes the `*`.

## Examples

### JavaScript

#### Don't

```js
function* ids() {
  return 1;
}
```

#### Fixed

```js
function ids() {
  return 1;
}
```

#### Do

```js
function* ids() {
  yield 1;
}
```

### JSX

#### Don't

```jsx
function* ids() {
  return 1;
}
```

#### Fixed

```jsx
function ids() {
  return 1;
}
```

#### Do

```jsx
function* ids() {
  yield 1;
}
```

### TypeScript

#### Don't

```ts
function* ids() {
  return 1;
}
```

#### Fixed

```ts
function ids() {
  return 1;
}
```

#### Do

```ts
function* ids() {
  yield 1;
}
```

### TSX

#### Don't

```tsx
function* ids() {
  return 1;
}
```

#### Fixed

```tsx
function ids() {
  return 1;
}
```

#### Do

```tsx
function* ids() {
  yield 1;
}
```
