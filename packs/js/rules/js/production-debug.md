---
id: js/production-debug
title: "No error-handler middleware that shows stack traces, unguarded"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which environments get the debug handler is the owner's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/production-debug]
since: 0.5.0
---

Express's `errorhandler` middleware, which sends stack traces to the client, is installed only behind a development check — not with a bare `app.use(errorhandler())`.

**Why:** Stack traces in production tell an attacker the code's structure, paths and versions.

```codelaw
match:
  kind: expression_statement
  pattern: "$A.use(errorhandler($$$O));"
  not:
    inside:
      stopBy: end
      kind: if_statement
message: "Install `errorhandler` only in development"
```

## How to fix

1. Wrap it in `if (process.env.NODE_ENV === "development")`, and use a handler that hides details elsewhere.

## Examples

### JavaScript

#### Don't

```js
app.use(errorhandler());
```

#### Do

```js
if (process.env.NODE_ENV === "development") {
  app.use(errorhandler());
}
```


### JSX

#### Don't

```jsx
app.use(errorhandler());
```

#### Do

```jsx
if (process.env.NODE_ENV === "development") {
  app.use(errorhandler());
}
```


### TypeScript

#### Don't

```ts
app.use(errorhandler());
```

#### Do

```ts
if (process.env.NODE_ENV === "development") {
  app.use(errorhandler());
}
```


### TSX

#### Don't

```tsx
app.use(errorhandler());
```

#### Do

```tsx
if (process.env.NODE_ENV === "development") {
  app.use(errorhandler());
}
```
