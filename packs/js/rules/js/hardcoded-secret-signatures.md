---
id: js/hardcoded-secret-signatures
title: "No hard-coded JWT secret"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the secret's new home is the owner's choice"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/hardcoded-secret-signatures]
since: 0.5.0
---

A JWT is not signed or verified with a string literal as the key.

**Why:** Anyone with the source can mint valid tokens.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)jwt\.(sign|verify)$|^(sign|verify)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 2
        kind: string
message: The signing secret is in the code; read it from a secret store
```

## How to fix

1. Load the key from the environment or a secret manager, and rotate the one that was committed.

## Examples

### JavaScript

#### Don't

```js
const t = jwt.sign(payload, "s3cr3t");
```

#### Do

```js
const t = jwt.sign(payload, process.env.JWT_SECRET);
```

### JSX

#### Don't

```jsx
const t = jwt.sign(payload, "s3cr3t");
```

#### Do

```jsx
const t = jwt.sign(payload, process.env.JWT_SECRET);
```

### TypeScript

#### Don't

```ts
const t = jwt.sign(payload, "s3cr3t");
```

#### Do

```ts
const t = jwt.sign(payload, process.env.JWT_SECRET);
```

### TSX

#### Don't

```tsx
const t = jwt.sign(payload, "s3cr3t");
```

#### Do

```tsx
const t = jwt.sign(payload, process.env.JWT_SECRET);
```
