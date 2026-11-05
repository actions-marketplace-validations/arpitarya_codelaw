---
id: js/promise-valid-params
title: "Promise methods get the number of arguments they take"
languages: [javascript, jsx, typescript, tsx]
severity: warn
fix: manual
fix_reason: "which argument is the intended one cannot be known from the call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-promise/valid-params]
since: 0.5.0
---

`Promise.resolve` and `Promise.reject` take at most one argument; `Promise.all`, `allSettled`, `race` and `any` exactly one; `.then()` one or two; `.catch()` and `.finally()` exactly one.

**Why:** An extra argument is silently ignored and a missing one makes the call do nothing useful — either way the code does not do what it says.

```codelaw
utils:
  none: { not: { has: { field: arguments, has: { nthChild: 1 } } } }
  two: { has: { field: arguments, has: { nthChild: 2 } } }
  three: { has: { field: arguments, has: { nthChild: 3 } } }
match:
  kind: call_expression
  any:
    - has: { field: function, regex: '^Promise\.(resolve|reject)$' }
      matches: two
    - has: { field: function, regex: '^Promise\.(all|allSettled|race|any)$' }
      any: [{matches: none}, {matches: two}]
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^then$' } }
      any: [{matches: none}, {matches: three}]
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^(catch|finally)$' } }
      any: [{matches: none}, {matches: two}]
message: "This promise method is called with the wrong number of arguments"
```

## How to fix

1. Pass the arguments the method takes: one value to `resolve`/`reject`, one iterable to `all`/`race`, one or two handlers to `then`, one to `catch`/`finally`.

## Examples

### JavaScript

#### Don't

```js
Promise.all(a, b);
```

#### Do

```js
Promise.all([a, b]);
```

### JSX

#### Don't

```jsx
Promise.all(a, b);
```

#### Do

```jsx
Promise.all([a, b]);
```

### TypeScript

#### Don't

```ts
Promise.all(a, b);
```

#### Do

```ts
Promise.all([a, b]);
```

### TSX

#### Don't

```tsx
Promise.all(a, b);
```

#### Do

```tsx
Promise.all([a, b]);
```
