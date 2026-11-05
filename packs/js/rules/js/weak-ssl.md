---
id: js/weak-ssl
title: "No TLS 1.0 or 1.1"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "very old clients will no longer connect"
stability: preview
ported_from: [eslint-plugin-sonarjs/weak-ssl]
since: 0.5.0
---

A TLS options object does not allow SSL 3, TLS 1.0 or TLS 1.1.

**Why:** These versions have known attacks and are disabled by modern clients.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(minVersion|secureProtocol)["']?$
  all:
  - has:
      field: value
      regex: ^.TLSv1(\.1)?.$|TLSv1(_1)?_method|SSLv[23]
message: This TLS version is broken; require TLS 1.2 or later
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
https.createServer({ key, cert, minVersion: "TLSv1" }, app);
```

#### Do

```js
https.createServer({ key, cert, minVersion: "TLSv1.2" }, app);
```

### JSX

#### Don't

```jsx
https.createServer({ key, cert, minVersion: "TLSv1" }, app);
```

#### Do

```jsx
https.createServer({ key, cert, minVersion: "TLSv1.2" }, app);
```

### TypeScript

#### Don't

```ts
https.createServer({ key, cert, minVersion: "TLSv1" }, app);
```

#### Do

```ts
https.createServer({ key, cert, minVersion: "TLSv1.2" }, app);
```

### TSX

#### Don't

```tsx
https.createServer({ key, cert, minVersion: "TLSv1" }, app);
```

#### Do

```tsx
https.createServer({ key, cert, minVersion: "TLSv1.2" }, app);
```
