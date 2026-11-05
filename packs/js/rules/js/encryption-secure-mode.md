---
id: js/encryption-secure-mode
title: "No ECB or unauthenticated CBC"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing the mode changes the data format"
stability: preview
ported_from: [eslint-plugin-sonarjs/encryption-secure-mode]
since: 0.5.0
---

AES is not used in ECB or CBC mode.

**Why:** ECB encrypts equal blocks to equal output; CBC without a MAC can be tampered with.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)(createCipheriv|createDecipheriv)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.(?i:aes-\d+-(ecb|cbc)|aes\d+).$
message: ECB leaks patterns and CBC has no integrity; use AES-GCM
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const c = crypto.createCipheriv("aes-128-ecb", key, null);
```

#### Do

```js
const c = crypto.createCipheriv("aes-128-gcm", key, iv);
```

### JSX

#### Don't

```jsx
const c = crypto.createCipheriv("aes-128-ecb", key, null);
```

#### Do

```jsx
const c = crypto.createCipheriv("aes-128-gcm", key, iv);
```

### TypeScript

#### Don't

```ts
const c = crypto.createCipheriv("aes-128-ecb", key, null);
```

#### Do

```ts
const c = crypto.createCipheriv("aes-128-gcm", key, iv);
```

### TSX

#### Don't

```tsx
const c = crypto.createCipheriv("aes-128-ecb", key, null);
```

#### Do

```tsx
const c = crypto.createCipheriv("aes-128-gcm", key, iv);
```
