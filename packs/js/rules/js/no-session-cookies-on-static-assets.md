---
id: js/no-session-cookies-on-static-assets
title: "Serve static files before the session middleware"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving middleware changes the order every request passes through, which the app's owner must check"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-session-cookies-on-static-assets]
since: 0.5.0
---

An Express app that registers `session(…)` before `express.static(…)` starts a session — and sends a session cookie — for every image, script and stylesheet. A cached asset then carries a cookie a shared cache may store and serve to someone else. `app.use(express.static(…))` after `app.use(session(…))` on the same app is reported.

**Why:** Session cookies on public assets can leak between users through caches, and cost a store write per file.

```codelaw
match:
  kind: expression_statement
  pattern: "$APP.use(express.static($$$));"
  follows:
    stopBy: end
    pattern: "$APP.use(session($$$));"
message: "Register static files before the session middleware"
```

## How to fix

1. Move `app.use(express.static(…))` above `app.use(session(…))`, so assets are served before a session starts.

## Examples

### JavaScript

#### Don't

```js
app.use(session({ secret }));
app.use(express.static("public"));
```

#### Do

```js
app.use(express.static("public"));
app.use(session({ secret }));
```


### JSX

#### Don't

```jsx
app.use(session({ secret }));
app.use(express.static("public"));
```

#### Do

```jsx
app.use(express.static("public"));
app.use(session({ secret }));
```


### TypeScript

#### Don't

```ts
app.use(session({ secret }));
app.use(express.static("public"));
```

#### Do

```ts
app.use(express.static("public"));
app.use(session({ secret }));
```


### TSX

#### Don't

```tsx
app.use(session({ secret }));
app.use(express.static("public"));
```

#### Do

```tsx
app.use(express.static("public"));
app.use(session({ secret }));
```
