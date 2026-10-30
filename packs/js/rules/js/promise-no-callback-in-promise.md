---
id: js/promise-no-callback-in-promise
title: "No node-style callback is called inside a then or catch"
languages: [javascript, jsx, typescript, tsx]
severity: warn
fix: manual
fix_reason: "moving the call out of the promise needs a choice between `nodeify`-style wrappers, `setImmediate` or rewriting the caller"
stability: preview
ported_from: [eslint-plugin-promise/no-callback-in-promise]
since: 0.5.0
---

A `.then()` or `.catch()` callback does not call `callback`, `cb`, `next` or `done`.

**Why:** Calling a callback from inside a promise runs it inside the promise's error handling: if it throws, the throw becomes a rejection the callback's caller never sees, and the callback may run twice.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: identifier, regex: '^(callback|cb|next|done)$' }
  inside:
    stopBy: end
    any: [{kind: arrow_function}, {kind: function_expression}]
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: '^(then|catch)$' } }
message: "Do not call a callback inside a promise callback; return the promise instead"
```

## How to fix

1. Return the promise to the caller, or call the callback outside the chain (for example in `setImmediate`) so an error it throws is not swallowed.

## Examples

### JavaScript

#### Don't

```js
function load(cb) {
  read().then((data) => cb(null, data));
}
```

#### Do

```js
function load() {
  return read();
}
```

### JSX

#### Don't

```jsx
function load(cb) {
  read().then((data) => cb(null, data));
}
```

#### Do

```jsx
function load() {
  return read();
}
```

### TypeScript

#### Don't

```ts
function load(cb) {
  read().then((data) => cb(null, data));
}
```

#### Do

```ts
function load() {
  return read();
}
```

### TSX

#### Don't

```tsx
function load(cb) {
  read().then((data) => cb(null, data));
}
```

#### Do

```tsx
function load() {
  return read();
}
```
