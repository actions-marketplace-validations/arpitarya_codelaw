---
id: js/aws-s3-bucket-versioning
title: "S3 buckets are versioned"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "versioning costs storage, which is the owner's trade-off; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-s3-bucket-versioning]
since: 0.5.0
---

An AWS CDK `Bucket` is not created with `versioned: false`.

**Why:** Without versions, an accidental or malicious overwrite is final.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(versioned)["']?$
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
message: Versioning is off; a deleted or overwritten object cannot be recovered
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
new s3.Bucket(this, "Data", { versioned: false });
```

#### Do

```js
new s3.Bucket(this, "Data", { versioned: true });
```

### JSX

#### Don't

```jsx
new s3.Bucket(this, "Data", { versioned: false });
```

#### Do

```jsx
new s3.Bucket(this, "Data", { versioned: true });
```

### TypeScript

#### Don't

```ts
new s3.Bucket(this, "Data", { versioned: false });
```

#### Do

```ts
new s3.Bucket(this, "Data", { versioned: true });
```

### TSX

#### Don't

```tsx
new s3.Bucket(this, "Data", { versioned: false });
```

#### Do

```tsx
new s3.Bucket(this, "Data", { versioned: true });
```
