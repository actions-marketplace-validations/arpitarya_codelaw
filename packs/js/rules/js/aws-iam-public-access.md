---
id: js/aws-iam-public-access
title: "No IAM policy for any principal"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which principals need access is the owner's call"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-iam-public-access]
since: 0.5.0
---

An AWS CDK policy does not use `AnyPrincipal` or `StarPrincipal`.

**Why:** A wildcard principal makes the resource public.

```codelaw

match:
  kind: new_expression
  has: { field: constructor, regex: '(^|\.)(AnyPrincipal|StarPrincipal)$' }
message: "This policy grants access to anyone, including anonymous users"
```

## How to fix

1. Name the account, role or service that needs access instead of any principal.

## Examples

### JavaScript

#### Don't

```js
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AnyPrincipal()] }));
```

#### Do

```js
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AccountRootPrincipal()] }));
```

### JSX

#### Don't

```jsx
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AnyPrincipal()] }));
```

#### Do

```jsx
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AccountRootPrincipal()] }));
```

### TypeScript

#### Don't

```ts
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AnyPrincipal()] }));
```

#### Do

```ts
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AccountRootPrincipal()] }));
```

### TSX

#### Don't

```tsx
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AnyPrincipal()] }));
```

#### Do

```tsx
bucket.addToResourcePolicy(new iam.PolicyStatement({ actions: ["s3:GetObject"], principals: [new iam.AccountRootPrincipal()] }));
```
