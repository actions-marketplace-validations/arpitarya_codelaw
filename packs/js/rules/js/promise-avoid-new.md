---
id: js/promise-avoid-new
title: "No new Promise"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "replacing an executor means choosing an `async` function or a promisified API"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-promise/avoid-new]
since: 0.5.0
---

Promises are made with `Promise.resolve`, `Promise.reject`, an `async` function or a promisified API, not with `new Promise`.

**Why:** A hand-written executor is where resolve-twice and forgotten-reject bugs live; most code that writes one is wrapping a callback API `util.promisify` already wraps.

```codelaw
match:
  kind: new_expression
  has: { field: constructor, regex: '^Promise$' }
message: "Avoid `new Promise`; use an `async` function or a promisified API"
```

## How to fix

1. Write the function as `async`, or wrap the callback API with `util.promisify`.

## Examples

### JavaScript

#### Don't

```js
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
```

#### Do

```js
const wait = promisify(setTimeout);
```

### JSX

#### Don't

```jsx
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
```

#### Do

```jsx
const wait = promisify(setTimeout);
```

### TypeScript

#### Don't

```ts
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
```

#### Do

```ts
const wait = promisify(setTimeout);
```

### TSX

#### Don't

```tsx
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
```

#### Do

```tsx
const wait = promisify(setTimeout);
```
