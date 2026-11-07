---
id: js/csrf
title: "CSRF protection does not skip state-changing methods"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which routes may go without the check is a security decision"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/csrf]
since: 0.5.0
---

`csurf`'s `ignoreMethods` lists the HTTP methods it does not check. Listing `POST`, `PUT`, `PATCH` or `DELETE` there turns the protection off for exactly the requests it exists for. Upstream also reports routes declared without the middleware; that needs the app's middleware order across the file, and is not reported here.

**Why:** A forged form post from another site goes through unchecked.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, regex: '^(csrf|csurf)$' }
    - has:
        field: arguments
        has:
          kind: object
          has:
            kind: pair
            has: { field: key, regex: '^["'']?ignoreMethods["'']?$' }
            regex: '["''](POST|PUT|PATCH|DELETE)["'']'
message: "CSRF protection is turned off for a state-changing method"
```

## How to fix

1. Keep `ignoreMethods` to the safe methods (`GET`, `HEAD`, `OPTIONS`), the default.

## Examples

### JavaScript

#### Don't

```js
const protect = csrf({ cookie: true, ignoreMethods: ["GET", "POST"] });
```

#### Do

```js
const protect = csrf({ cookie: true });
```


### JSX

#### Don't

```jsx
const protect = csrf({ cookie: true, ignoreMethods: ["GET", "POST"] });
```

#### Do

```jsx
const protect = csrf({ cookie: true });
```


### TypeScript

#### Don't

```ts
const protect = csrf({ cookie: true, ignoreMethods: ["GET", "POST"] });
```

#### Do

```ts
const protect = csrf({ cookie: true });
```


### TSX

#### Don't

```tsx
const protect = csrf({ cookie: true, ignoreMethods: ["GET", "POST"] });
```

#### Do

```tsx
const protect = csrf({ cookie: true });
```
