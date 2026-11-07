---
id: js/session-regeneration
title: "Regenerate the session after passport login"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "where the regeneration goes, and what session data must survive it, is the app's to decide"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/session-regeneration]
since: 0.5.0
---

A route that logs a user in with `passport.authenticate(…)` and keeps the session it had before is open to session fixation: an attacker who planted that session id is now logged in too. A route whose handler after `passport.authenticate` never calls `req.session.regenerate` is reported; `{ session: false }` is left.

**Why:** The session id must change when the user's privileges do.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, kind: member_expression, has: { field: property, regex: '^(post|get|put|all|use)$' } }
    - has:
        field: arguments
        all:
          - has:
              kind: call_expression
              regex: '^passport\.authenticate\('
              not: { regex: 'session\s*:\s*false' }
          - has:
              any: [{ kind: function_expression }, { kind: arrow_function }]
              not: { has: { stopBy: end, regex: '\.session\.regenerate$' } }
message: "Regenerate the session once the user is logged in"
```

## How to fix

1. In the handler after `passport.authenticate`, call `req.session.regenerate(…)` and carry over only what the new session needs.

## Examples

### JavaScript

#### Don't

```js
app.post("/login", passport.authenticate("local"), (req, res) => {
  res.redirect("/");
});
```

#### Do

```js
app.post("/login", passport.authenticate("local"), (req, res, next) => {
  req.session.regenerate((err) => (err ? next(err) : res.redirect("/")));
});
```


### JSX

#### Don't

```jsx
app.post("/login", passport.authenticate("local"), (req, res) => {
  res.redirect("/");
});
```

#### Do

```jsx
app.post("/login", passport.authenticate("local"), (req, res, next) => {
  req.session.regenerate((err) => (err ? next(err) : res.redirect("/")));
});
```


### TypeScript

#### Don't

```ts
app.post("/login", passport.authenticate("local"), (req, res) => {
  res.redirect("/");
});
```

#### Do

```ts
app.post("/login", passport.authenticate("local"), (req, res, next) => {
  req.session.regenerate((err) => (err ? next(err) : res.redirect("/")));
});
```


### TSX

#### Don't

```tsx
app.post("/login", passport.authenticate("local"), (req, res) => {
  res.redirect("/");
});
```

#### Do

```tsx
app.post("/login", passport.authenticate("local"), (req, res, next) => {
  req.session.regenerate((err) => (err ? next(err) : res.redirect("/")));
});
```
