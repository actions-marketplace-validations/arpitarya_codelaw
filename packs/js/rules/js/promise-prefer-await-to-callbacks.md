---
id: js/promise-prefer-await-to-callbacks
title: "async and await, not callbacks"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning a callback API into a promise changes every caller"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-promise/prefer-await-to-callbacks]
since: 0.5.0
---

A function does not take or call a `callback` or `cb` parameter: it returns a promise and its callers `await` it.

**Why:** Callbacks spread error handling across every call site and do not compose with `try`/`catch`.

```codelaw
match:
  any:
    - kind: call_expression
      has: { field: function, kind: identifier, regex: '^(callback|cb)$' }
    - any: [{kind: identifier}, {kind: required_parameter}, {kind: optional_parameter}]
      regex: '^(callback|cb)\b'
      nthChild: { position: 1, reverse: true }
      inside: { kind: formal_parameters }
message: "Return a promise and use `await` instead of a callback"
```

## How to fix

1. Make the function `async` (or return a promise), drop the callback parameter, and `await` it at each call site.

## Examples

### JavaScript

#### Don't

```js
function load(path, cb) {
  read(path, cb);
}
```

#### Do

```js
async function load(path) {
  return read(path);
}
```

### JSX

#### Don't

```jsx
function load(path, cb) {
  read(path, cb);
}
```

#### Do

```jsx
async function load(path) {
  return read(path);
}
```

### TypeScript

#### Don't

```ts
function load(path, cb) {
  read(path, cb);
}
```

#### Do

```ts
async function load(path) {
  return read(path);
}
```

### TSX

#### Don't

```tsx
function load(path, cb) {
  read(path, cb);
}
```

#### Do

```tsx
async function load(path) {
  return read(path);
}
```
