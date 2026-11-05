---
id: js/no-weak-cipher
title: "No weak cipher"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing the cipher changes the key size and the data format"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-weak-cipher]
since: 0.5.0
---

A cipher is not created with DES, 3DES, RC2, RC4 or Blowfish.

**Why:** These ciphers have short keys or known attacks.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)(createCipheriv|createDecipheriv|createCipher)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.(?i:des|des3|des-ede|des-ede3|rc2|rc4|bf|blowfish)[-a-z0-9]*.$
message: This cipher is weak; use AES-GCM
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const c = crypto.createCipheriv("des-cbc", key, iv);
```

#### Do

```js
const c = crypto.createCipheriv("aes-256-gcm", key, iv);
```

### JSX

#### Don't

```jsx
const c = crypto.createCipheriv("des-cbc", key, iv);
```

#### Do

```jsx
const c = crypto.createCipheriv("aes-256-gcm", key, iv);
```

### TypeScript

#### Don't

```ts
const c = crypto.createCipheriv("des-cbc", key, iv);
```

#### Do

```ts
const c = crypto.createCipheriv("aes-256-gcm", key, iv);
```

### TSX

#### Don't

```tsx
const c = crypto.createCipheriv("des-cbc", key, iv);
```

#### Do

```tsx
const c = crypto.createCipheriv("aes-256-gcm", key, iv);
```
