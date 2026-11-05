---
id: js/aws-s3-bucket-insecure-http
title: "S3 buckets enforce TLS"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "clients that still use HTTP must change first; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-s3-bucket-insecure-http]
since: 0.5.0
---

An AWS CDK `Bucket` is not created with `enforceSSL: false`.

**Why:** Requests over HTTP can be read and changed in transit.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(enforceSSL)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)Bucket$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: 'This bucket accepts plain HTTP requests; set `enforceSSL: true`'
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
new s3.Bucket(this, "Logs", { enforceSSL: false });
```

#### Do

```js
new s3.Bucket(this, "Logs", { enforceSSL: true });
```

### JSX

#### Don't

```jsx
new s3.Bucket(this, "Logs", { enforceSSL: false });
```

#### Do

```jsx
new s3.Bucket(this, "Logs", { enforceSSL: true });
```

### TypeScript

#### Don't

```ts
new s3.Bucket(this, "Logs", { enforceSSL: false });
```

#### Do

```ts
new s3.Bucket(this, "Logs", { enforceSSL: true });
```

### TSX

#### Don't

```tsx
new s3.Bucket(this, "Logs", { enforceSSL: false });
```

#### Do

```tsx
new s3.Bucket(this, "Logs", { enforceSSL: true });
```
