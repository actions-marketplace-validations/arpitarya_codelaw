---
id: js/promise-no-return-wrap
title: "A then or catch callback returns the value, not Promise.resolve or Promise.reject"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning `return Promise.reject(e)` into `throw e` changes the statement's shape inside code the author may want to restructure"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint-plugin-promise/no-return-wrap]
since: 0.5.0
---

Inside a `.then()` or `.catch()` callback, return the value itself rather than `Promise.resolve(value)`, and `throw` the error rather than return `Promise.reject(error)`.

**Why:** The callback's return value is already wrapped in a promise, so the extra wrapping is noise, and a returned rejection hides a `throw` the reader would see at once.

```codelaw
utils:
  callback:
    any: [{kind: arrow_function}, {kind: function_expression}]
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: '^(then|catch)$' } }
match:
  any:
    - kind: return_statement
      has:
        kind: call_expression
        regex: '^Promise\.(resolve|reject)\s*\('
      inside:
        matches: callback
        stopBy:
          any: [{kind: arrow_function}, {kind: function_expression}, {kind: function_declaration}, {kind: method_definition}]
    - kind: call_expression
      regex: '^Promise\.(resolve|reject)\s*\('
      inside: { matches: callback, field: body }
message: "Return the value (or throw the error) instead of wrapping it in `Promise.resolve` or `Promise.reject`"
```

## How to fix

1. Replace `return Promise.resolve(x)` with `return x`.
2. Replace `return Promise.reject(e)` with `throw e`.

## Examples

### JavaScript

#### Don't

```js
fetchUser().then((user) => {
  return Promise.resolve(user.name);
});
```

#### Do

```js
fetchUser().then((user) => {
  return user.name;
});
```

### JSX

#### Don't

```jsx
fetchUser().then((user) => {
  return Promise.resolve(user.name);
});
```

#### Do

```jsx
fetchUser().then((user) => {
  return user.name;
});
```

### TypeScript

#### Don't

```ts
fetchUser().then((user) => {
  return Promise.resolve(user.name);
});
```

#### Do

```ts
fetchUser().then((user) => {
  return user.name;
});
```

### TSX

#### Don't

```tsx
fetchUser().then((user) => {
  return Promise.resolve(user.name);
});
```

#### Do

```tsx
fetchUser().then((user) => {
  return user.name;
});
```
