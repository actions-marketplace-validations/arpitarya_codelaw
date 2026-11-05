---
id: js/aws-rds-unencrypted-databases
title: "RDS storage is encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "encrypting an existing database means a migration; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-rds-unencrypted-databases]
since: 0.5.0
---

An AWS CDK RDS instance or cluster is not created with `storageEncrypted: false`.

**Why:** Unencrypted database storage exposes data through disks and snapshots.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(storageEncrypted)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)(DatabaseInstance|DatabaseCluster|CfnDBInstance|CfnDBCluster)$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: This database is not encrypted at rest
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: false });
```

#### Do

```js
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: true });
```

### JSX

#### Don't

```jsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: false });
```

#### Do

```jsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: true });
```

### TypeScript

#### Don't

```ts
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: false });
```

#### Do

```ts
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: true });
```

### TSX

#### Don't

```tsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: false });
```

#### Do

```tsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, storageEncrypted: true });
```
