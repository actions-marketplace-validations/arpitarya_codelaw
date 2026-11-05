---
id: js/catch-error-name
title: "Name the caught error `error`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "renaming can collide with an outer `error` the handler also reads"
stability: preview
ported_from: [eslint-plugin-unicorn/catch-error-name]
since: 0.5.0
---

The parameter of a `catch` clause is named `error`; `_`, and names ending in `Error` (`fsError`), are left alone.

**Why:** One name for the caught value makes every handler read the same, and `e` or `ex` says nothing.

```codelaw

match:
  kind: catch_clause
  has: { field: parameter, kind: identifier, pattern: $N, not: { regex: '^(_|error|err_?|.*Error)$' } }
message: "Name the caught error `error`, not `$N`"
fix:
  - rename-symbol-local: { target: $N, to: error }
```

## Examples

### JavaScript

#### Don't

```js
try {
  run();
} catch (e) {
  log(e);
}
```

#### Do

```js
try {
  run();
} catch (error) {
  log(error);
}
```

### JSX

#### Don't

```jsx
try {
  run();
} catch (e) {
  log(e);
}
```

#### Do

```jsx
try {
  run();
} catch (error) {
  log(error);
}
```

### TypeScript

#### Don't

```ts
try {
  run();
} catch (e) {
  log(e);
}
```

#### Do

```ts
try {
  run();
} catch (error) {
  log(error);
}
```

### TSX

#### Don't

```tsx
try {
  run();
} catch (e) {
  log(e);
}
```

#### Do

```tsx
try {
  run();
} catch (error) {
  log(error);
}
```
