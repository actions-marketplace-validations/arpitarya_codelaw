---
id: js/aws-s3-bucket-granted-access
title: "No public ACL on an S3 bucket"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the access is intended is the owner's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-s3-bucket-granted-access]
since: 0.5.0
---

An AWS CDK `Bucket` is not given a `PUBLIC_READ`, `PUBLIC_READ_WRITE` or `AUTHENTICATED_READ` canned ACL.

**Why:** These ACLs open the bucket beyond the account.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(accessControl)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)Bucket$
  all:
  - has:
      field: value
      regex: (PUBLIC_READ|PUBLIC_READ_WRITE|AUTHENTICATED_READ)$
message: This ACL grants access to everyone, or to every AWS account
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PUBLIC_READ });
```

#### Do

```js
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PRIVATE });
```

### JSX

#### Don't

```jsx
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PUBLIC_READ });
```

#### Do

```jsx
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PRIVATE });
```

### TypeScript

#### Don't

```ts
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PUBLIC_READ });
```

#### Do

```ts
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PRIVATE });
```

### TSX

#### Don't

```tsx
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PUBLIC_READ });
```

#### Do

```tsx
new s3.Bucket(this, "B", { accessControl: s3.BucketAccessControl.PRIVATE });
```
