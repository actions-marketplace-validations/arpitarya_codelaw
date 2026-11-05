---
id: js/aws-sqs-unencrypted-queue
title: "SQS queues are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a KMS key has its own cost and policy"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-sqs-unencrypted-queue]
since: 0.5.0
---

An AWS CDK `Queue` is not created with `QueueEncryption.UNENCRYPTED`.

**Why:** Messages at rest are readable by anyone with access to the storage.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(encryption)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)Queue$
  all:
  - has:
      field: value
      regex: UNENCRYPTED$
message: This queue stores messages unencrypted
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.UNENCRYPTED });
```

#### Do

```js
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.SQS_MANAGED });
```

### JSX

#### Don't

```jsx
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.UNENCRYPTED });
```

#### Do

```jsx
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.SQS_MANAGED });
```

### TypeScript

#### Don't

```ts
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.UNENCRYPTED });
```

#### Do

```ts
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.SQS_MANAGED });
```

### TSX

#### Don't

```tsx
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.UNENCRYPTED });
```

#### Do

```tsx
new sqs.Queue(this, "Jobs", { encryption: sqs.QueueEncryption.SQS_MANAGED });
```
