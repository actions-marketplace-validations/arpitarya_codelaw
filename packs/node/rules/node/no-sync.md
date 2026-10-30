---
id: node/no-sync
title: "No synchronous *Sync calls"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the async form returns a promise or takes a callback, so the calling code has to become async"
stability: preview
ported_from: [eslint-plugin-n/no-sync]
since: 0.5.0
---

Code calls the asynchronous form of an API, not a method whose name ends in `Sync` (upstream default: reported at the top level too).

**Why:** A `*Sync` call blocks the event loop until the I/O finishes; in a server every request waits behind it.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has: { field: property, regex: ^\w+Sync$, pattern: $M }
message: "`$M` blocks the event loop; use the asynchronous version"
```

## How to fix

1. Call the promise form (`fs.promises.readFile`, `await`) or the callback form instead, and make the caller async.
2. In a CLI or build script where blocking is fine, keep it and exclude the file from the rule.

## Examples

### JavaScript

#### Don't

```js
function load(path) {
  return fs.readFileSync(path, "utf8");
}
```

#### Do

```js
async function load(path) {
  return fs.promises.readFile(path, "utf8");
}
```

### JSX

#### Don't

```jsx
function load(path) {
  return fs.readFileSync(path, "utf8");
}
```

#### Do

```jsx
async function load(path) {
  return fs.promises.readFile(path, "utf8");
}
```

### TypeScript

#### Don't

```ts
function load(path) {
  return fs.readFileSync(path, "utf8");
}
```

#### Do

```ts
async function load(path) {
  return fs.promises.readFile(path, "utf8");
}
```

### TSX

#### Don't

```tsx
function load(path) {
  return fs.readFileSync(path, "utf8");
}
```

#### Do

```tsx
async function load(path) {
  return fs.promises.readFile(path, "utf8");
}
```
