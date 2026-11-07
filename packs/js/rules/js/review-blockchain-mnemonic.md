---
id: js/review-blockchain-mnemonic
title: "No wallet seed phrase written in the code"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a committed seed phrase is compromised; the wallet must be rotated, which no edit does"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/review-blockchain-mnemonic]
since: 0.5.0
---

A seed phrase gives full control of every key derived from it. A string literal (or a template without substitutions) of eight or more words passed to `fromPhrase`, `fromMnemonic` or `mnemonicToAccount` is reported — and it is compromised once committed, test phrases included.

**Why:** Anyone who reads the repository, or its history, owns the wallet.

```codelaw
match:
  kind: call_expression
  all:
    - has: { field: function, regex: '(^|\.)(fromPhrase|fromMnemonic|mnemonicToAccount)$' }
    - has:
        field: arguments
        has:
          nthChild: 1
          any:
            - kind: string
            - kind: template_string
              not: { has: { kind: template_substitution } }
          regex: '^.\s*[A-Za-z]+(\s+[A-Za-z]+){7,}\s*.$'
message: "Revoke this seed phrase and load it from a secret store"
```

## How to fix

1. Move the funds to a new wallet: the phrase is compromised.
2. Read the new phrase from the environment or a secret manager at run time.

## Examples

### JavaScript

#### Don't

```js
const wallet = Wallet.fromPhrase("candy sugar pudding cream honey rich smooth crumble sweet treat");
```

#### Do

```js
const wallet = Wallet.fromPhrase(process.env.WALLET_PHRASE);
```


### JSX

#### Don't

```jsx
const wallet = Wallet.fromPhrase("candy sugar pudding cream honey rich smooth crumble sweet treat");
```

#### Do

```jsx
const wallet = Wallet.fromPhrase(process.env.WALLET_PHRASE);
```


### TypeScript

#### Don't

```ts
const wallet = Wallet.fromPhrase("candy sugar pudding cream honey rich smooth crumble sweet treat");
```

#### Do

```ts
const wallet = Wallet.fromPhrase(process.env.WALLET_PHRASE);
```


### TSX

#### Don't

```tsx
const wallet = Wallet.fromPhrase("candy sugar pudding cream honey rich smooth crumble sweet treat");
```

#### Do

```tsx
const wallet = Wallet.fromPhrase(process.env.WALLET_PHRASE);
```
