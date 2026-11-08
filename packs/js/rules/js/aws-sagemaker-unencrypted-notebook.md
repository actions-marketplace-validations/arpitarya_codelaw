---
id: js/aws-sagemaker-unencrypted-notebook
title: "SageMaker notebooks are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a KMS key has its own cost and policy"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-sagemaker-unencrypted-notebook]
since: 0.5.0
---

An AWS CDK `CfnNotebookInstance` is created with a `kmsKeyId`.

**Why:** Without one, the notebook's storage volume is not encrypted with a key the account controls.

```codelaw
match:
  kind: new_expression
  has: { field: constructor, regex: '(^|\.)CfnNotebookInstance$' }
  not:
    has:
      field: arguments
      any:
        - has: { kind: object, has: { kind: pair, has: { field: key, regex: '^["'']?kmsKeyId["'']?$' } } }
        - has: { kind: spread_element }
        - has: { kind: identifier, nthChild: 3 }
message: "This notebook's volume is not encrypted with a KMS key"
```

## How to fix

1. Pass `kmsKeyId` with the key that should encrypt the notebook's volume.

## Examples

### JavaScript

#### Don't

```js
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn });
```

#### Do

```js
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn, kmsKeyId: key.keyId });
```


### JSX

#### Don't

```jsx
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn });
```

#### Do

```jsx
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn, kmsKeyId: key.keyId });
```


### TypeScript

#### Don't

```ts
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn });
```

#### Do

```ts
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn, kmsKeyId: key.keyId });
```


### TSX

#### Don't

```tsx
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn });
```

#### Do

```tsx
new sagemaker.CfnNotebookInstance(this, "Nb", { instanceType: "ml.t3.medium", roleArn, kmsKeyId: key.keyId });
```
