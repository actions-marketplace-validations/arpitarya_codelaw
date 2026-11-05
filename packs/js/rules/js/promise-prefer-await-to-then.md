---
id: js/promise-prefer-await-to-then
title: "await, not then"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a chain to `await` needs an `async` function and may restructure the error handling"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-promise/prefer-await-to-then, eslint-plugin-unicorn/prefer-await]
since: 0.5.0
---

A promise's result is taken with `await`, not `.then()`.

**Why:** `await` reads top to bottom, keeps values in scope without nesting, and works with `try`/`catch`.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^then$' } }
  not: { inside: { kind: await_expression } }
message: "Use `await` instead of `.then()`"
```

## How to fix

1. Make the enclosing function `async` and replace `p.then((x) => …)` with `const x = await p;`.

## Examples

### JavaScript

#### Don't

```js
function show() {
  return load().then((data) => render(data));
}
```

#### Do

```js
async function show() {
  const data = await load();
  return render(data);
}
```

### JSX

#### Don't

```jsx
function show() {
  return load().then((data) => render(data));
}
```

#### Do

```jsx
async function show() {
  const data = await load();
  return render(data);
}
```

### TypeScript

#### Don't

```ts
function show() {
  return load().then((data) => render(data));
}
```

#### Do

```ts
async function show() {
  const data = await load();
  return render(data);
}
```

### TSX

#### Don't

```tsx
function show() {
  return load().then((data) => render(data));
}
```

#### Do

```tsx
async function show() {
  const data = await load();
  return render(data);
}
```
