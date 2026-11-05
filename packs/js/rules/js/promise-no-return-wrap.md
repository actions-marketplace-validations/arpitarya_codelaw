---
id: js/promise-no-return-wrap
title: "A then or catch callback returns the value, not Promise.resolve or Promise.reject"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "`return Promise.resolve(x)` is `return x` and `return Promise.reject(e)` is `throw e`; only a callback with a try around the return, where the new throw would be caught, stays a suggestion"
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
      any:
        - all: [{ pattern: $RRES }, { has: { pattern: Promise.resolve($V1) } }]
        - all: [{ pattern: $RREJ }, { has: { pattern: Promise.reject($V2) } }]
        - kind: return_statement
    - kind: call_expression
      regex: '^Promise\.(resolve|reject)\s*\('
      inside: { matches: callback, field: body }
      any:
        - all: [{ pattern: $CRES }, { pattern: Promise.resolve($V3) }]
        - all: [{ pattern: $CREJ }, { pattern: Promise.reject($V4) }]
        - kind: call_expression
message: "Return the value (or throw the error) instead of wrapping it in `Promise.resolve` or `Promise.reject`"
auto_when:
  not: { inside: { kind: try_statement, stopBy: { any: [{kind: arrow_function}, {kind: function_expression}, {kind: function_declaration}, {kind: method_definition}] } } }
fix:
  - replace: { target: $RRES, with: "return $V1;" }
  - replace: { target: $RREJ, with: "throw $V2;" }
  - replace: { target: $CRES, with: "$V3" }
  - replace: { target: $CREJ, with: "{ throw $V4; }" }
```

## How to fix

1. Replace `return Promise.resolve(x)` with `return x`, and `return Promise.reject(e)` with `throw e`; the fix does both, and reports a suggestion where a `try` in the callback would catch the new `throw`.

## Examples

### JavaScript

#### Don't

```js
fetchUser().then((user) => {
  return Promise.resolve(user.name);
});
```

#### Fixed

```js
fetchUser().then((user) => {
  return user.name;
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

#### Fixed

```jsx
fetchUser().then((user) => {
  return user.name;
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

#### Fixed

```ts
fetchUser().then((user) => {
  return user.name;
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

#### Fixed

```tsx
fetchUser().then((user) => {
  return user.name;
});
```

#### Do

```tsx
fetchUser().then((user) => {
  return user.name;
});
```
