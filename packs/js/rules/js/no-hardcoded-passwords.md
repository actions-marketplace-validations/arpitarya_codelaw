---
id: js/no-hardcoded-passwords
title: "No password in the code"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the secret's new home is the author's choice"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-hardcoded-passwords]
since: 0.5.0
---

A variable, property or assignment whose name contains `password`, `passwd`, `pwd` or `passphrase` is not given a non-empty string literal.

**Why:** Anyone with the source has the password, and it cannot be rotated without a release.

```codelaw
match:
  any:
  - kind: variable_declarator
    has:
      field: name
      regex: (?i)(password|passwd|pwd|passphrase)
    all:
    - has:
        field: value
        kind: string
        regex: ^.[^\s]{1,}.$
  - kind: pair
    has:
      field: key
      regex: (?i)(password|passwd|pwd|passphrase)
    all:
    - has:
        field: value
        kind: string
        regex: ^.[^\s]{1,}.$
  - kind: assignment_expression
    has:
      field: left
      regex: (?i)(password|passwd|pwd|passphrase)$
    all:
    - has:
        field: right
        kind: string
        regex: ^.[^\s]{1,}.$
message: A password is written in the code; read it from a secret store or the environment
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const dbPassword = "hunter2";
```

#### Do

```js
const dbPassword = process.env.DB_PASSWORD;
```

### JSX

#### Don't

```jsx
const dbPassword = "hunter2";
```

#### Do

```jsx
const dbPassword = process.env.DB_PASSWORD;
```

### TypeScript

#### Don't

```ts
const dbPassword = "hunter2";
```

#### Do

```ts
const dbPassword = process.env.DB_PASSWORD;
```

### TSX

#### Don't

```tsx
const dbPassword = "hunter2";
```

#### Do

```tsx
const dbPassword = process.env.DB_PASSWORD;
```
