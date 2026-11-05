---
id: sec-js/jwt-verify
title: "A JWT is verified with a named algorithm, never `none`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "naming the algorithm is fail-closed, but the reviewer confirms it is the one the issuer signs with (RS256, HS256 or ES256)"
stability: preview
security: { cwe: [347], owasp: ["A07:2025"], severity: high }
since: 0.3.0
---

`jwt.verify` is given an `algorithms` list, no algorithm is `none`, and `jwt.decode` is not the whole check.

**Why:** Without `algorithms` a token can pick its own: `none` carries no signature, and an RSA public key used as an HMAC secret lets anyone mint tokens (CWE-347). `decode` reads a token without checking it.

```codelaw
match:
  any:
    - pattern: $J.verify($T, $K)
    - all:
        - pattern: $J2.verify($T2, $K2, $O)
        - not:
            has:
              stopBy: end
              regex: 'algorithms'
    - all:
        - kind: pair
        - regex: '(?i)^["'']?algorithms?["'']?\s*:\s*\[?\s*["'']none["'']'
    - all:
        - pattern: $J3.decode($T3)
        - not:
            inside:
              kind: program
              stopBy: end
              has:
                stopBy: end
                regex: '\.verify\(|jwtVerify|verifyJwt'
constraints:
  J: { regex: '^(jwt|jsonwebtoken|jws)$' }
  J2: { regex: '^(jwt|jsonwebtoken|jws)$' }
  J3: { regex: '^(jwt|jsonwebtoken|jws|jwtDecode)$' }
message: "This JWT check lets the token choose its algorithm, accepts `none`, or does not verify"
fix:
  choices:
    - label: RS256
      fix:
        - insert-after: {target: $K, text: ', { algorithms: ["RS256"] }'}
        - replace: {target: $O, with: '{ ...$O, algorithms: ["RS256"] }'}
    - label: HS256
      fix:
        - insert-after: {target: $K, text: ', { algorithms: ["HS256"] }'}
        - replace: {target: $O, with: '{ ...$O, algorithms: ["HS256"] }'}
    - label: ES256
      fix:
        - insert-after: {target: $K, text: ', { algorithms: ["ES256"] }'}
        - replace: {target: $O, with: '{ ...$O, algorithms: ["ES256"] }'}
```

## How to fix

1. Pass `{ algorithms: ["RS256"] }` (or the one algorithm you issue with) to `jwt.verify`.
2. Never put `none` in the list, and never use a public key as an HMAC secret.
3. Use `jwt.decode` only to read a token that `jwt.verify` has already accepted.

## Examples

### JavaScript

#### Don't

```js
const claims = jwt.verify(token, publicKey);
```

#### Fixed (RS256)

```js
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

#### Fixed (HS256)

```js
const claims = jwt.verify(token, publicKey, { algorithms: ["HS256"] });
```

#### Fixed (ES256)

```js
const claims = jwt.verify(token, publicKey, { algorithms: ["ES256"] });
```

#### Do

```js
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

### JSX

#### Don't

```jsx
const claims = jwt.verify(token, publicKey);
```

#### Fixed (RS256)

```jsx
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

#### Fixed (HS256)

```jsx
const claims = jwt.verify(token, publicKey, { algorithms: ["HS256"] });
```

#### Fixed (ES256)

```jsx
const claims = jwt.verify(token, publicKey, { algorithms: ["ES256"] });
```

#### Do

```jsx
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

### TypeScript

#### Don't

```ts
const claims = jwt.verify(token, publicKey);
```

#### Fixed (RS256)

```ts
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

#### Fixed (HS256)

```ts
const claims = jwt.verify(token, publicKey, { algorithms: ["HS256"] });
```

#### Fixed (ES256)

```ts
const claims = jwt.verify(token, publicKey, { algorithms: ["ES256"] });
```

#### Do

```ts
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

### TSX

#### Don't

```tsx
const claims = jwt.verify(token, publicKey);
```

#### Fixed (RS256)

```tsx
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```

#### Fixed (HS256)

```tsx
const claims = jwt.verify(token, publicKey, { algorithms: ["HS256"] });
```

#### Fixed (ES256)

```tsx
const claims = jwt.verify(token, publicKey, { algorithms: ["ES256"] });
```

#### Do

```tsx
const claims = jwt.verify(token, publicKey, { algorithms: ["RS256"] });
```
