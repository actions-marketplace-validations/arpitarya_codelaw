---
id: js/aws-iam-all-resources-accessible
title: "No IAM policy on every resource"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which resources are needed is the owner's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-iam-all-resources-accessible]
since: 0.5.0
---

An AWS CDK `PolicyStatement` does not grant `resources: ["*"]`.

**Why:** A wildcard resource extends the permission to everything the account creates later.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(resources)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)PolicyStatement$
  all:
  - has:
      field: value
      regex: ^\[\s*.\*.\s*\]$
message: This statement applies to every resource; name the ones it needs
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: ["*"] });
```

#### Do

```js
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### JSX

#### Don't

```jsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: ["*"] });
```

#### Do

```jsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### TypeScript

#### Don't

```ts
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: ["*"] });
```

#### Do

```ts
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### TSX

#### Don't

```tsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: ["*"] });
```

#### Do

```tsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```
