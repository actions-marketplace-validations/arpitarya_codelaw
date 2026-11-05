---
id: js/hashing
title: "No MD5 or SHA-1"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing the algorithm changes every stored digest"
stability: preview
ported_from: [eslint-plugin-sonarjs/hashing]
since: 0.5.0
---

`createHash` and `createHmac` are not called with a weak algorithm (`md2`, `md4`, `md5`, `sha1`, `ripemd160`).

**Why:** Collisions for these can be computed, so a signature or integrity check based on them can be forged.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)(createHash|createHmac)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.(?i:md2|md4|md5|sha1|sha-1|ripemd160).$
message: MD5 and SHA-1 are broken; use SHA-256 or better (and a password hash such as scrypt for passwords)
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const digest = crypto.createHash("md5").update(data).digest("hex");
```

#### Do

```js
const digest = crypto.createHash("sha256").update(data).digest("hex");
```

### JSX

#### Don't

```jsx
const digest = crypto.createHash("md5").update(data).digest("hex");
```

#### Do

```jsx
const digest = crypto.createHash("sha256").update(data).digest("hex");
```

### TypeScript

#### Don't

```ts
const digest = crypto.createHash("md5").update(data).digest("hex");
```

#### Do

```ts
const digest = crypto.createHash("sha256").update(data).digest("hex");
```

### TSX

#### Don't

```tsx
const digest = crypto.createHash("md5").update(data).digest("hex");
```

#### Do

```tsx
const digest = crypto.createHash("sha256").update(data).digest("hex");
```
