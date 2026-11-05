---
id: js/aws-opensearchservice-domain
title: "OpenSearch domains are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "encryption cannot be turned on for an existing domain"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-opensearchservice-domain]
since: 0.5.0
---

An AWS CDK OpenSearch `Domain` is not created with `encryptionAtRest: { enabled: false }`.

**Why:** Unencrypted index data is exposed through disks and snapshots.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(encryptionAtRest)["']?$
  all:
  - has:
      field: value
      regex: enabled:\s*false
message: This OpenSearch domain is not encrypted at rest
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: false } });
```

#### Do

```js
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: true } });
```

### JSX

#### Don't

```jsx
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: false } });
```

#### Do

```jsx
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: true } });
```

### TypeScript

#### Don't

```ts
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: false } });
```

#### Do

```ts
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: true } });
```

### TSX

#### Don't

```tsx
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: false } });
```

#### Do

```tsx
new opensearch.Domain(this, "Search", { version, encryptionAtRest: { enabled: true } });
```
