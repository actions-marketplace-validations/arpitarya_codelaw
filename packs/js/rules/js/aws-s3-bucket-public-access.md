---
id: js/aws-s3-bucket-public-access
title: "No public-read S3 bucket"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "whether the data is meant to be public is the owner's call; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-s3-bucket-public-access]
since: 0.5.0
---

An AWS CDK `Bucket` is not created with `publicReadAccess: true`. Upstream also checks `blockPublicAccess`; this port checks the explicit setting.

**Why:** A public bucket exposes every object in it.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(publicReadAccess)["']?$
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
      regex: ^true$
message: This bucket is readable by anyone on the internet
fix:
- replace:
    target: $V
    with: 'false'
```

## Examples

### JavaScript

#### Don't

```js
new s3.Bucket(this, "Site", { publicReadAccess: true });
```

#### Do

```js
new s3.Bucket(this, "Site", { publicReadAccess: false });
```

### JSX

#### Don't

```jsx
new s3.Bucket(this, "Site", { publicReadAccess: true });
```

#### Do

```jsx
new s3.Bucket(this, "Site", { publicReadAccess: false });
```

### TypeScript

#### Don't

```ts
new s3.Bucket(this, "Site", { publicReadAccess: true });
```

#### Do

```ts
new s3.Bucket(this, "Site", { publicReadAccess: false });
```

### TSX

#### Don't

```tsx
new s3.Bucket(this, "Site", { publicReadAccess: true });
```

#### Do

```tsx
new s3.Bucket(this, "Site", { publicReadAccess: false });
```
