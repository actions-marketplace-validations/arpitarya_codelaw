---
id: js/no-secrets
title: "No string that looks like a secret"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the secret moves to the environment or a secret store, and the code reads it from there"
fix_exception: security
stability: preview
options:
  entropyThreshold: { type: "number", default: 4.1, description: "bits of Shannon entropy per character above which a long string with no spaces is reported" }
  minLength: { type: "number", default: 24, description: "the shortest string the entropy check reads" }
ported_from: [biome/security/noSecrets, eslint-plugin-no-secrets/no-secrets]
since: 0.5.1
---

A string literal is reported when it has the shape of a known credential — a JSON Web Token, a Slack token or webhook URL, a GitHub token, an AWS access key (`AKIA…`), a Twilio key (`SK…`), a password inside a URL, a private key block, a Google service account — or, when it is `minLength` characters or more with no whitespace, its Shannon entropy is above `entropyThreshold` bits per character (Biome's default of 4.1). Module specifiers, paths and URLs are not read by the entropy check, and a template with a `${…}` is skipped. Behaviour of Biome's `noSecrets`; `sec-js/no-hardcoded-secret` is the rule for what a name says (`password = "…"`).

**Why:** A secret in the source is in every clone, every fork and every build of the repository, for good.

```codelaw
native: js/no-secrets
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const key = "AKIAIOSFODNN7EXAMPLE";
```

#### Do

```js
const key = process.env.AWS_ACCESS_KEY_ID;
```

### JSX

#### Don't

```jsx
const key = "AKIAIOSFODNN7EXAMPLE";
```

#### Do

```jsx
const key = process.env.AWS_ACCESS_KEY_ID;
```

### TypeScript

#### Don't

```ts
const key = "AKIAIOSFODNN7EXAMPLE";
```

#### Do

```ts
const key = process.env.AWS_ACCESS_KEY_ID;
```

### TSX

#### Don't

```tsx
const key = "AKIAIOSFODNN7EXAMPLE";
```

#### Do

```tsx
const key = process.env.AWS_ACCESS_KEY_ID;
```
