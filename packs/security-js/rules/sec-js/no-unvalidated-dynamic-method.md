---
id: sec-js/no-unvalidated-dynamic-method
title: "No method chosen by a request value"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an own-key check blocks prototype methods and keeps `this`, but the reviewer confirms the own keys are the ones meant to be callable"
stability: preview
ported_from: [eslint-plugin-security/detect-object-injection]
security: { cwe: [470], owasp: ["A05:2025"], severity: high }
since: 0.3.0
---

A function is not looked up on an object by a request value and then called: `obj[req.query.fn]()`.

**Why:** The client picks which method runs, including ones the author never meant to expose: `constructor`, `__proto__` or `toString` (CWE-470).

```codelaw
match:
  pattern: $O[$K]($$$ARGS)
where:
  $K: { from_source: { tables: [sources/express, sources/koa, sources/nextjs] } }
message: "`$K` comes from the request and chooses the method that is called"
fix: "(Object.hasOwn($O, $K) ? $O[$K] : null).call($O, $$$ARGS)"
```

## How to fix

1. Compare the value with each allowed name and call the function by name.
2. Or look it up in a table that has no prototype (`Object.create(null)`, a `Map`) after `Object.hasOwn(table, key)`.

## Examples

### JavaScript

#### Don't

```js
handlers[req.query.action](req, res);
```

#### Fixed

```js
(Object.hasOwn(handlers, req.query.action) ? handlers[req.query.action] : null).call(handlers, req, res);
```

#### Do

```js
if (req.query.action === "list") {
  list(req, res);
}
```

### JSX

#### Don't

```jsx
handlers[req.query.action](req, res);
```

#### Fixed

```jsx
(Object.hasOwn(handlers, req.query.action) ? handlers[req.query.action] : null).call(handlers, req, res);
```

#### Do

```jsx
if (req.query.action === "list") {
  list(req, res);
}
```

### TypeScript

#### Don't

```ts
handlers[req.query.action](req, res);
```

#### Fixed

```ts
(Object.hasOwn(handlers, req.query.action) ? handlers[req.query.action] : null).call(handlers, req, res);
```

#### Do

```ts
if (req.query.action === "list") {
  list(req, res);
}
```

### TSX

#### Don't

```tsx
handlers[req.query.action](req, res);
```

#### Fixed

```tsx
(Object.hasOwn(handlers, req.query.action) ? handlers[req.query.action] : null).call(handlers, req, res);
```

#### Do

```tsx
if (req.query.action === "list") {
  list(req, res);
}
```
