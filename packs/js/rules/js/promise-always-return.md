---
id: js/promise-always-return
title: "A then callback returns or throws"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the callback should return is the author's to decide"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-promise/always-return]
since: 0.5.0
---

A `.then()` callback with a block body returns a value or throws.

**Why:** The next link of the chain receives what the callback returns; a callback that returns nothing passes `undefined` on, which is almost always a forgotten `return` before an inner promise.

```codelaw
match:
  any: [{kind: arrow_function}, {kind: function_expression}]
  nthChild: 1
  has:
    field: body
    kind: statement_block
    not:
      has:
        any: [{kind: return_statement}, {kind: throw_statement}]
        stopBy: end
  inside:
    kind: arguments
    inside:
      kind: call_expression
      has: { field: function, kind: member_expression, has: { field: property, regex: '^then$' } }
message: "Return a value or throw from this `.then()` callback"
```

## How to fix

1. Return the value the next step needs — often the inner promise the callback started — or throw if the callback found an error.

## Examples

### JavaScript

#### Don't

```js
load().then((data) => {
  store(data);
}).catch(report);
```

#### Do

```js
load().then((data) => {
  return store(data);
}).catch(report);
```

### JSX

#### Don't

```jsx
load().then((data) => {
  store(data);
}).catch(report);
```

#### Do

```jsx
load().then((data) => {
  return store(data);
}).catch(report);
```

### TypeScript

#### Don't

```ts
load().then((data) => {
  store(data);
}).catch(report);
```

#### Do

```ts
load().then((data) => {
  return store(data);
}).catch(report);
```

### TSX

#### Don't

```tsx
load().then((data) => {
  store(data);
}).catch(report);
```

#### Do

```tsx
load().then((data) => {
  return store(data);
}).catch(report);
```

## Differences from the source

The source follows each code path; this rule asks only whether the body has a `return` or `throw` anywhere, so a callback that returns on one branch and falls off the end of another is not reported.
