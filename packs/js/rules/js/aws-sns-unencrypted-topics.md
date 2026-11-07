---
id: js/aws-sns-unencrypted-topics
title: "SNS topics are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a KMS key has its own cost and policy"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-sns-unencrypted-topics]
since: 0.5.0
---

An AWS CDK SNS `Topic` is created with a `masterKey`, and a `CfnTopic` with a `kmsMasterKeyId`.

**Why:** Without one, messages at rest are not encrypted with a key the account controls.

```codelaw
utils:
  keyed:
    kind: object
    has: { kind: pair, has: { field: key, regex: '^["'']?(masterKey|kmsMasterKeyId)["'']?$' } }
match:
  kind: new_expression
  has: { field: constructor, regex: '(^|\.)(Topic|CfnTopic)$' }
  not:
    has:
      field: arguments
      any:
        - has: { matches: keyed }
        - has: { kind: spread_element }
        - has: { kind: identifier, nthChild: 3 }
message: "This topic stores messages without a KMS key"
```

## How to fix

1. Pass `masterKey` (a `kms.Key`) to the topic, or `kmsMasterKeyId` to `CfnTopic`.

## Examples

### JavaScript

#### Don't

```js
new sns.Topic(this, "Alerts", { displayName: "alerts" });
```

#### Do

```js
new sns.Topic(this, "Alerts", { displayName: "alerts", masterKey: key });
```


### JSX

#### Don't

```jsx
new sns.Topic(this, "Alerts", { displayName: "alerts" });
```

#### Do

```jsx
new sns.Topic(this, "Alerts", { displayName: "alerts", masterKey: key });
```


### TypeScript

#### Don't

```ts
new sns.Topic(this, "Alerts", { displayName: "alerts" });
```

#### Do

```ts
new sns.Topic(this, "Alerts", { displayName: "alerts", masterKey: key });
```


### TSX

#### Don't

```tsx
new sns.Topic(this, "Alerts", { displayName: "alerts" });
```

#### Do

```tsx
new sns.Topic(this, "Alerts", { displayName: "alerts", masterKey: key });
```
