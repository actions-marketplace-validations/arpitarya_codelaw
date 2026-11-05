---
id: js/no-hardcoded-secrets
title: "No secret in a variable name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the secret's new home is the owner's choice"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-hardcoded-secrets]
since: 0.5.0
---

A variable or property whose name ends in `secret`, `token`, `apiKey`, `accessKey`, `privateKey` or `authKey` is not given a string of 16 or more key characters.

**Why:** A committed secret is readable by everyone with the repository and stays in its history.

```codelaw
match:
  any:
  - kind: variable_declarator
    has:
      field: name
      regex: (?i)(secret|token|api_?key|access_?key|private_?key|auth_?key)$
    all:
    - has:
        field: value
        kind: string
        regex: ^.[A-Za-z0-9+/=_\-]{16,}.$
  - kind: pair
    has:
      field: key
      regex: (?i)(secret|token|api_?key|access_?key|private_?key|auth_?key).?$
    all:
    - has:
        field: value
        kind: string
        regex: ^.[A-Za-z0-9+/=_\-]{16,}.$
message: A secret is written in the code; read it from a secret store or the environment
```

## How to fix

1. Move the value to a secret store or the environment, read it at runtime, and rotate the committed one.

## Examples

### JavaScript

#### Don't

```js
const apiKey = "sk_live_4eC39HqLyjWDarjtT1zdp7dc";
```

#### Do

```js
const apiKey = process.env.STRIPE_KEY;
```

### JSX

#### Don't

```jsx
const apiKey = "sk_live_4eC39HqLyjWDarjtT1zdp7dc";
```

#### Do

```jsx
const apiKey = process.env.STRIPE_KEY;
```

### TypeScript

#### Don't

```ts
const apiKey = "sk_live_4eC39HqLyjWDarjtT1zdp7dc";
```

#### Do

```ts
const apiKey = process.env.STRIPE_KEY;
```

### TSX

#### Don't

```tsx
const apiKey = "sk_live_4eC39HqLyjWDarjtT1zdp7dc";
```

#### Do

```tsx
const apiKey = process.env.STRIPE_KEY;
```
