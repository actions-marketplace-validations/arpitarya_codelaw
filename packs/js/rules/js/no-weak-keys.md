---
id: js/no-weak-keys
title: "No RSA or DSA key shorter than 2048 bits"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a longer key changes what the other side must accept; the owner decides"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-weak-keys]
since: 0.5.0
---

A key pair made with Node's `crypto.generateKeyPair` or `generateKeyPairSync` has a `modulusLength` of at least 2048 bits.

**Why:** Keys shorter than 2048 bits can be factored with resources that are within reach today.

```codelaw
match:
  kind: pair
  has: { field: key, regex: '^["'']?modulusLength["'']?$' }
  all:
    - has: { field: value, kind: number, regex: '^(\d{1,3}|1\d{3}|20[0-3]\d|204[0-7])$' }
  inside:
    stopBy: end
    kind: call_expression
    has: { field: function, regex: '(^|\.)generateKeyPair(Sync)?$' }
message: "Use a modulus of at least 2048 bits"
```

## How to fix

1. Set `modulusLength` to 2048 or more (3072 for keys meant to last), and rotate keys already issued.

## Examples

### JavaScript

#### Don't

```js
crypto.generateKeyPairSync("rsa", { modulusLength: 1024 });
```

#### Do

```js
crypto.generateKeyPairSync("rsa", { modulusLength: 3072 });
```


### JSX

#### Don't

```jsx
crypto.generateKeyPairSync("rsa", { modulusLength: 1024 });
```

#### Do

```jsx
crypto.generateKeyPairSync("rsa", { modulusLength: 3072 });
```


### TypeScript

#### Don't

```ts
crypto.generateKeyPairSync("rsa", { modulusLength: 1024 });
```

#### Do

```ts
crypto.generateKeyPairSync("rsa", { modulusLength: 3072 });
```


### TSX

#### Don't

```tsx
crypto.generateKeyPairSync("rsa", { modulusLength: 1024 });
```

#### Do

```tsx
crypto.generateKeyPairSync("rsa", { modulusLength: 3072 });
```
