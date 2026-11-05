---
id: js/cors
title: "No wildcard CORS origin"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the allowed origins are the owner's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/cors]
since: 0.5.0
---

A CORS configuration does not allow every origin (`origin: "*"` or `origin: true`).

**Why:** A wildcard lets any page read the responses, including authenticated ones when credentials are allowed.

```codelaw
match:
  kind: pair
  inside:
    kind: object
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has:
          field: function
          regex: (^|[.])cors$
  has:
    field: key
    regex: ^["']?(origin|"Access-Control-Allow-Origin")["']?$
  all:
  - has:
      field: value
      regex: ^.\*.$|^true$
message: Any site can read responses from this server; list the allowed origins
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
app.use(cors({ origin: "*" }));
```

#### Do

```js
app.use(cors({ origin: ["https://app.example.com"] }));
```

### JSX

#### Don't

```jsx
app.use(cors({ origin: "*" }));
```

#### Do

```jsx
app.use(cors({ origin: ["https://app.example.com"] }));
```

### TypeScript

#### Don't

```ts
app.use(cors({ origin: "*" }));
```

#### Do

```ts
app.use(cors({ origin: ["https://app.example.com"] }));
```

### TSX

#### Don't

```tsx
app.use(cors({ origin: "*" }));
```

#### Do

```tsx
app.use(cors({ origin: ["https://app.example.com"] }));
```
