---
id: node/no-async-express-handler
title: "No async Express route handlers"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "forwarding the rejection needs the project's own wrapper or error middleware, whose name is not in this file"
fix_exception: value
stability: preview
ported_from: [oxlint/no-async-endpoint-handlers]
since: 0.5.0
---

An Express route or middleware (`app.get`, `router.post`, `app.use`, ...) is not an `async` function taking `(req, res, ...)`. A handler with fewer than two parameters is not reported.

**Why:** Express 4 ignores the promise a handler returns: a rejection is never passed to the error middleware, and the request hangs or the process logs an unhandled rejection.

```codelaw
match:
  any: [{kind: arrow_function}, {kind: function_expression}]
  regex: ^async\b
  has: { field: parameters, has: { nthChild: 2 } }
  inside:
    kind: arguments
    inside:
      kind: call_expression
      has:
        field: function
        kind: member_expression
        has: { field: property, regex: ^(get|post|put|patch|delete|all|use|options|head)$ }
message: "Express won't catch this handler's rejections; wrap it so errors reach `next`"
```

## How to fix

1. Wrap the handler: `app.get(path, (req, res, next) => handler(req, res).catch(next))`, or use the project's async wrapper.
2. On Express 5, which forwards rejections itself, exclude the files from this rule.

## Examples

### JavaScript

#### Don't

```js
app.get("/users", async (req, res) => {
  res.json(await db.users());
});
```

#### Do

```js
app.get("/users", (req, res, next) => {
  db.users().then((users) => res.json(users), next);
});
```

### JSX

#### Don't

```jsx
app.get("/users", async (req, res) => {
  res.json(await db.users());
});
```

#### Do

```jsx
app.get("/users", (req, res, next) => {
  db.users().then((users) => res.json(users), next);
});
```

### TypeScript

#### Don't

```ts
app.get("/users", async (req, res) => {
  res.json(await db.users());
});
```

#### Do

```ts
app.get("/users", (req, res, next) => {
  db.users().then((users) => res.json(users), next);
});
```

### TSX

#### Don't

```tsx
app.get("/users", async (req, res) => {
  res.json(await db.users());
});
```

#### Do

```tsx
app.get("/users", (req, res, next) => {
  db.users().then((users) => res.json(users), next);
});
```
