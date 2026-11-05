---
id: js/unverified-certificate
title: "Verify TLS certificates"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the right fix is a trusted CA, which is configuration outside the code; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/unverified-certificate]
since: 0.5.0
---

A TLS option object does not set `rejectUnauthorized: false`.

**Why:** Turning off verification accepts any certificate, so the connection is not authenticated.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^(rejectUnauthorized|"rejectUnauthorized")$
  all:
  - has:
      field: value
      pattern: $V
      kind: 'false'
message: Certificate checks are off; anyone on the network can impersonate the server
fix:
- remove
```

## Examples

### JavaScript

#### Don't

```js
https.request({ host, rejectUnauthorized: false });
```

#### Do

```js
https.request({ host });
```

### JSX

#### Don't

```jsx
https.request({ host, rejectUnauthorized: false });
```

#### Do

```jsx
https.request({ host });
```

### TypeScript

#### Don't

```ts
https.request({ host, rejectUnauthorized: false });
```

#### Do

```ts
https.request({ host });
```

### TSX

#### Don't

```tsx
https.request({ host, rejectUnauthorized: false });
```

#### Do

```tsx
https.request({ host });
```
