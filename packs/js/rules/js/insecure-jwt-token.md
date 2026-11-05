---
id: js/insecure-jwt-token
title: "No unsigned JWTs"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the algorithm must match how tokens are issued"
stability: preview
ported_from: [eslint-plugin-sonarjs/insecure-jwt-token]
since: 0.5.0
---

A JWT sign or verify option does not name the `none` algorithm.

**Why:** An unsigned token can be forged by anyone.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(algorithm|algorithms)["']?$
  all:
  - has:
      field: value
      regex: .none.
message: The `none` algorithm accepts tokens with no signature
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
jwt.verify(token, key, { algorithms: ["none"] });
```

#### Do

```js
jwt.verify(token, key, { algorithms: ["RS256"] });
```

### JSX

#### Don't

```jsx
jwt.verify(token, key, { algorithms: ["none"] });
```

#### Do

```jsx
jwt.verify(token, key, { algorithms: ["RS256"] });
```

### TypeScript

#### Don't

```ts
jwt.verify(token, key, { algorithms: ["none"] });
```

#### Do

```ts
jwt.verify(token, key, { algorithms: ["RS256"] });
```

### TSX

#### Don't

```tsx
jwt.verify(token, key, { algorithms: ["none"] });
```

#### Do

```tsx
jwt.verify(token, key, { algorithms: ["RS256"] });
```
