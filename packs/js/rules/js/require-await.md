---
id: js/require-await
title: "An async function awaits something"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "dropping `async` changes the function to return its value directly, which callers may not expect; the reviewer confirms, since an `await` the author forgot is the other fix"
stability: preview
ported_from: [eslint/require-await]
since: 0.5.0
---

An `async` function with a non-empty body contains an `await` or a `for await` loop.

**Why:** An `async` function without `await` only wraps its result in a promise, which usually means an `await` was forgotten.

```codelaw

match:
  any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}]
  regex: '^(export\s+)?(default\s+)?(static\s+)?async\s'
  all:
    - has: { pattern: $$ASYNC, regex: '^async$' }
  has: { field: body, kind: statement_block, regex: "[^\\s{}]" }
  not: { has: { stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}] }, any: [{kind: await_expression}, {kind: for_in_statement, regex: '^for\s+await'}] } }
message: "This async function never awaits; drop `async`, or await what it calls"
fix:
  - remove: $$ASYNC
```

## How to fix

1. If the function does wait on something, add the `await`; otherwise remove `async` and check its callers do not rely on a promise; the fix removes `async`.

## Examples

### JavaScript

#### Don't

```js
async function load() {
  return fetch(url);
}
```

#### Fixed

```js
 function load() {
  return fetch(url);
}
```

#### Do

```js
function load() {
  return fetch(url);
}
```

### JSX

#### Don't

```jsx
async function load() {
  return fetch(url);
}
```

#### Fixed

```jsx
 function load() {
  return fetch(url);
}
```

#### Do

```jsx
function load() {
  return fetch(url);
}
```

### TypeScript

#### Don't

```ts
async function load() {
  return fetch(url);
}
```

#### Fixed

```ts
 function load() {
  return fetch(url);
}
```

#### Do

```ts
function load() {
  return fetch(url);
}
```

### TSX

#### Don't

```tsx
async function load() {
  return fetch(url);
}
```

#### Fixed

```tsx
 function load() {
  return fetch(url);
}
```

#### Do

```tsx
function load() {
  return fetch(url);
}
```
