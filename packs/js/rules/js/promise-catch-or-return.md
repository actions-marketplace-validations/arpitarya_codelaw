---
id: js/promise-catch-or-return
title: "A promise chain is returned, awaited or ends in catch"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the error should go — a `.catch()`, a `return` or an `await` — is the author's decision"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-promise/catch-or-return]
since: 0.5.0
---

A promise chain used as a statement on its own ends with `.catch()`; otherwise it is returned or awaited so its caller handles a rejection.

**Why:** A rejection in a chain nobody returns, awaits or catches becomes an unhandled rejection — in Node, a crash.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      any:
        - has: { field: property, regex: '^then$' }
        - all:
            - has: { field: property, regex: '^finally$' }
            - not: { regex: '\.catch\s*\(' }
message: "Handle this promise chain's rejection: end it with `.catch()`, or return or await it"
```

## How to fix

1. Add a `.catch()` at the end of the chain that handles or reports the error, or `return` / `await` the chain so the caller does.

## Examples

### JavaScript

#### Don't

```js
save(doc).then(() => notify());
```

#### Do

```js
save(doc)
  .then(() => notify())
  .catch((err) => report(err));
```

### JSX

#### Don't

```jsx
save(doc).then(() => notify());
```

#### Do

```jsx
save(doc)
  .then(() => notify())
  .catch((err) => report(err));
```

### TypeScript

#### Don't

```ts
save(doc).then(() => notify());
```

#### Do

```ts
save(doc)
  .then(() => notify())
  .catch((err) => report(err));
```

### TSX

#### Don't

```tsx
save(doc).then(() => notify());
```

#### Do

```tsx
save(doc)
  .then(() => notify())
  .catch((err) => report(err));
```
