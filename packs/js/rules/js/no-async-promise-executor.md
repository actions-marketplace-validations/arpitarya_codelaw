---
id: js/no-async-promise-executor
title: "A Promise executor is not an async function"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "dropping `async` sends a thrown error to the promise instead of an unhandled rejection, which the author confirms; offered only when the executor has no `await`"
stability: preview
ported_from: [eslint/no-async-promise-executor]
since: 0.1.0
---

The function passed to `new Promise(...)` is not `async`.

**Why:** An error thrown in an async executor doesn't reject the promise, and an executor that needs
`await` doesn't need the `Promise` wrapper.

```codelaw
match:
  any:
    - all:
        - { pattern: $W1 }
        - pattern: new Promise(async ($$$P1) => $$$B1)
        - not: { regex: '\bawait\b' }
    - all:
        - { pattern: $W2 }
        - pattern: new Promise(async function ($$$P2) { $$$B2 })
        - not: { regex: '\bawait\b' }
    - all:
        - { pattern: $W3 }
        - pattern: new Promise(async $P3 => $$$B3)
        - not: { regex: '\bawait\b' }
    - pattern: new Promise(async ($$$P) => $$$B)
    - pattern: new Promise(async function ($$$P) { $$$B })
    - pattern: new Promise(async $P => $$$B)
message: "An async Promise executor loses its errors; await inside a plain async function instead"
fix:
  - replace: { target: $W1, with: "new Promise(($$$P1) => $$$B1)" }
  - replace: { target: $W2, with: "new Promise(function ($$$P2) { $$$B2 })" }
  - replace: { target: $W3, with: "new Promise($P3 => $$$B3)" }
```

## How to fix

1. If the executor only awaits and resolves, use the awaited promise directly.
2. Otherwise make the executor a plain function and handle errors with `reject`.

## Examples

### JavaScript

#### Don't

```js
const data = new Promise(async (resolve) => {
  resolve(load());
});
```

#### Fixed

```js
const data = new Promise((resolve) => {
  resolve(load());
});
```

#### Do

```js
const data = load();
```

### JSX

#### Don't

```jsx
const data = new Promise(async (resolve) => {
  resolve(load());
});
```

#### Fixed

```jsx
const data = new Promise((resolve) => {
  resolve(load());
});
```

#### Do

```jsx
const data = load();
```

### TypeScript

#### Don't

```ts
const data = new Promise(async (resolve) => {
  resolve(load());
});
```

#### Fixed

```ts
const data = new Promise((resolve) => {
  resolve(load());
});
```

#### Do

```ts
const data = load();
```

### TSX

#### Don't

```tsx
const data = new Promise(async (resolve) => {
  resolve(load());
});
```

#### Fixed

```tsx
const data = new Promise((resolve) => {
  resolve(load());
});
```

#### Do

```tsx
const data = load();
```
