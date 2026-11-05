---
id: js/aws-iam-all-privileges
title: "No IAM policy granting every action"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which actions are needed is the owner's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-iam-all-privileges]
since: 0.5.0
---

An AWS CDK `PolicyStatement` does not grant `actions: ["*"]`.

**Why:** Every action includes deleting and changing permissions, so a leaked credential owns the account.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(actions)["']?$
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
message: This statement allows every action; list the actions it needs
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
new iam.PolicyStatement({ actions: ["*"], resources: [bucket.arnForObjects("*")] });
```

#### Do

```js
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### JSX

#### Don't

```jsx
new iam.PolicyStatement({ actions: ["*"], resources: [bucket.arnForObjects("*")] });
```

#### Do

```jsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### TypeScript

#### Don't

```ts
new iam.PolicyStatement({ actions: ["*"], resources: [bucket.arnForObjects("*")] });
```

#### Do

```ts
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```

### TSX

#### Don't

```tsx
new iam.PolicyStatement({ actions: ["*"], resources: [bucket.arnForObjects("*")] });
```

#### Do

```tsx
new iam.PolicyStatement({ actions: ["s3:GetObject"], resources: [bucket.arnForObjects("*")] });
```
