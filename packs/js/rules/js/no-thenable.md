---
id: js/no-thenable
title: "No then property on an object or class"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-thenable]
since: 0.5.0
---

An object literal, a class or a module does not define a member called `then`.

**Why:** `await` and `Promise.resolve` call any `then` they find, so the object is silently treated as a promise.

```codelaw

match:
  any:
    - { kind: pair, has: { field: key, regex: '^(then|"then"|''then'')$' } }
    - { kind: method_definition, has: { field: name, regex: '^then$' } }
    - { kind: export_statement, has: { stopBy: end, kind: function_declaration, has: { field: name, regex: '^then$' } } }
message: "An object with `then` is treated as a promise by `await`; rename it"
```

## How to fix

1. Rename the member, and its callers.

## Examples

### JavaScript

#### Don't

```js
const job = {
  then(cb) {
    cb();
  },
};
```

#### Do

```js
const job = {
  next(cb) {
    cb();
  },
};
```

### JSX

#### Don't

```jsx
const job = {
  then(cb) {
    cb();
  },
};
```

#### Do

```jsx
const job = {
  next(cb) {
    cb();
  },
};
```

### TypeScript

#### Don't

```ts
const job = {
  then(cb) {
    cb();
  },
};
```

#### Do

```ts
const job = {
  next(cb) {
    cb();
  },
};
```

### TSX

#### Don't

```tsx
const job = {
  then(cb) {
    cb();
  },
};
```

#### Do

```tsx
const job = {
  next(cb) {
    cb();
  },
};
```
