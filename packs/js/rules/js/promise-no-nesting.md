---
id: js/promise-no-nesting
title: "Promise chains are flattened, not nested"
languages: [javascript, jsx, typescript, tsx]
severity: warn
fix: suggest
fix_reason: "hoisting the inner `.then` onto the outer chain is only equivalent when its callback uses nothing the outer callback declared, which the reviewer confirms"
stability: preview
ported_from: [eslint-plugin-promise/no-nesting]
since: 0.5.0
---

A `.then()` or `.catch()` is not called inside another `.then()` or `.catch()` callback: return the inner promise and chain on it.

**Why:** Nested chains rebuild the callback pyramid promises were meant to remove, and a rejection inside one is easy to lose.

```codelaw
match:
  kind: call_expression
  any:
    - pattern: $B.then($I)
      inside:
        kind: arrow_function
        nthChild: 1
        inside:
          kind: arguments
          inside:
            kind: call_expression
            pattern: $OUTER
            has: { field: function, kind: member_expression, has: { field: property, regex: '^then$' } }
    - kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^(then|catch)$' } }
  inside:
    stopBy: end
    any: [{kind: arrow_function}, {kind: function_expression}]
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: '^(then|catch)$' } }
message: "Avoid nesting promise chains; return the inner promise and chain the next step"
fix:
  - replace: { target: match, with: "$B" }
  - insert-after: { target: $OUTER, text: ".then($I)" }
```

## How to fix

1. Return the inner promise from the outer callback and move its `.then()` up to the outer chain; the fix does this for a single-argument `.then` that is the arrow callback's whole body, and the reviewer checks the moved callback uses nothing the outer callback declared.

## Examples

### JavaScript

#### Don't

```js
getUser().then((user) =>
  getPosts(user).then((posts) => show(posts)),
);
```

#### Fixed

```js
getUser().then((user) =>
  getPosts(user),
).then((posts) => show(posts));
```

#### Do

```js
getUser()
  .then((user) => getPosts(user))
  .then((posts) => show(posts));
```

### JSX

#### Don't

```jsx
getUser().then((user) =>
  getPosts(user).then((posts) => show(posts)),
);
```

#### Fixed

```jsx
getUser().then((user) =>
  getPosts(user),
).then((posts) => show(posts));
```

#### Do

```jsx
getUser()
  .then((user) => getPosts(user))
  .then((posts) => show(posts));
```

### TypeScript

#### Don't

```ts
getUser().then((user) =>
  getPosts(user).then((posts) => show(posts)),
);
```

#### Fixed

```ts
getUser().then((user) =>
  getPosts(user),
).then((posts) => show(posts));
```

#### Do

```ts
getUser()
  .then((user) => getPosts(user))
  .then((posts) => show(posts));
```

### TSX

#### Don't

```tsx
getUser().then((user) =>
  getPosts(user).then((posts) => show(posts)),
);
```

#### Fixed

```tsx
getUser().then((user) =>
  getPosts(user),
).then((posts) => show(posts));
```

#### Do

```tsx
getUser()
  .then((user) => getPosts(user))
  .then((posts) => show(posts));
```

## Differences from the source

The source allows a nested chain whose callback uses the outer callback's parameters; this rule reports every nested chain.
