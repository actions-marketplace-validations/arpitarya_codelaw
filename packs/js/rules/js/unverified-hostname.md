---
id: js/unverified-hostname
title: "Verify the TLS host name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right check is the default one, or a pinned identity the author chooses"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/unverified-hostname]
since: 0.5.0
---

A TLS option object does not override `checkServerIdentity` with a function that returns nothing.

**Why:** Such a function accepts a certificate for any host name.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^checkServerIdentity$
  all:
  - has:
      field: value
      any:
      - kind: arrow_function
      - kind: function_expression
      not:
        has:
          stopBy: end
          kind: return_statement
          has:
            regex: .
      regex: =>\s*(\{\s*\}|undefined|null)|\{\s*\}$
message: This `checkServerIdentity` accepts every host; remove it or check the name
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
tls.connect({ host, checkServerIdentity: () => {} });
```

#### Do

```js
tls.connect({ host });
```

### JSX

#### Don't

```jsx
tls.connect({ host, checkServerIdentity: () => {} });
```

#### Do

```jsx
tls.connect({ host });
```

### TypeScript

#### Don't

```ts
tls.connect({ host, checkServerIdentity: () => {} });
```

#### Do

```ts
tls.connect({ host });
```

### TSX

#### Don't

```tsx
tls.connect({ host, checkServerIdentity: () => {} });
```

#### Do

```tsx
tls.connect({ host });
```
