---
id: js/require-atomic-updates
title: "No assignment from a value read before an `await` or `yield`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which read to move after the `await` (or whether the update should be serialised) is the author's"
fix_exception: restructure
stability: preview
options: { allowProperties: { type: "boolean", default: false, description: "do not report assignments to a property (`object.property = …`), only to the variable itself" } }
ported_from: [eslint/require-atomic-updates]
since: 0.5.0
---

An assignment inside an `async` or generator function that stores into a variable other functions can reach, or into a property of one, is reported when the variable was read before an `await` or `yield` that the right-hand side contains or follows: `total += await load()` and `total = total + await load()` read `total`, wait, and write back a stale value if another call changed it meanwhile. A variable only the function itself uses, one read again after the last `await`, an assignment to a property of an object reached only through the function's own locals, and an undeclared name are left alone. `allowProperties: true` reports only the variable itself, not `object.property = …`. Like ESLint's code-path graph, a loop's second pass is not followed, and a `catch` clause starts from where its `try` block first can throw, where it ends and each `throw`. Behaviour of ESLint 10.13.0 (read, not copied).

**Why:** Between the read and the write another call can run and change the value, and the write then undoes it: an update lost without an error.

```codelaw
native: js/require-atomic-updates
```

## How to fix

1. Read the variable after the `await`: await into a local first, then update the shared value in one step, or serialise the calls that write it.

## Examples

### JavaScript

#### Don't

```js
let total = 0;

async function add(id) {
  total += await load(id);
}
```

#### Do

```js
let total = 0;

async function add(id) {
  const part = await load(id);
  total += part;
}
```


### JSX

#### Don't

```jsx
let total = 0;

async function add(id) {
  total += await load(id);
}
```

#### Do

```jsx
let total = 0;

async function add(id) {
  const part = await load(id);
  total += part;
}
```


### TypeScript

#### Don't

```ts
let total = 0;

async function add(id) {
  total += await load(id);
}
```

#### Do

```ts
let total = 0;

async function add(id) {
  const part = await load(id);
  total += part;
}
```


### TSX

#### Don't

```tsx
let total = 0;

async function add(id) {
  total += await load(id);
}
```

#### Do

```tsx
let total = 0;

async function add(id) {
  const part = await load(id);
  total += part;
}
```
