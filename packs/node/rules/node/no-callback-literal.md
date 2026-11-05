---
id: node/no-callback-literal
title: "Pass an Error, not a string, to the callback"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "wrapping the message in `new Error(...)` changes the value the callback receives, from a string to an Error"
stability: preview
ported_from: [eslint-plugin-n/no-callback-literal]
since: 0.5.0
---

A call to `callback` or `cb` does not pass a string or template literal as its error argument. Upstream also reports numbers, booleans and object literals there; those have no one right rewrite (an error or a result?), so this port reports only strings.

**Why:** Error-first callbacks expect an `Error` (or `null`); a string has no stack trace and breaks `err instanceof Error` and `err.message`.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: identifier, regex: ^(callback|cb)$ }
  all:
    - has:
        field: arguments
        has:
          nthChild: 1
          any: [{kind: string}, {kind: template_string}]
          pattern: $A
message: "Pass `new Error($A)` as the callback's error, not a bare string"
fix:
  - replace: { target: $A, with: "new Error($A)" }
```

## Examples

### JavaScript

#### Don't

```js
function find(id, cb) {
  if (!id) return cb("missing id");
  lookup(id, cb);
}
```

#### Do

```js
function find(id, cb) {
  if (!id) return cb(new Error("missing id"));
  lookup(id, cb);
}
```

### JSX

#### Don't

```jsx
function find(id, cb) {
  if (!id) return cb("missing id");
  lookup(id, cb);
}
```

#### Do

```jsx
function find(id, cb) {
  if (!id) return cb(new Error("missing id"));
  lookup(id, cb);
}
```

### TypeScript

#### Don't

```ts
function find(id, cb) {
  if (!id) return cb("missing id");
  lookup(id, cb);
}
```

#### Do

```ts
function find(id, cb) {
  if (!id) return cb(new Error("missing id"));
  lookup(id, cb);
}
```

### TSX

#### Don't

```tsx
function find(id, cb) {
  if (!id) return cb("missing id");
  lookup(id, cb);
}
```

#### Do

```tsx
function find(id, cb) {
  if (!id) return cb(new Error("missing id"));
  lookup(id, cb);
}
```
