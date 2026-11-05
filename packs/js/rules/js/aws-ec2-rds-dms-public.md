---
id: js/aws-ec2-rds-dms-public
title: "No publicly accessible instance or database"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "whether public access is needed is the owner's call; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-ec2-rds-dms-public]
since: 0.5.0
---

An AWS CDK EC2 instance, RDS database or DMS replication instance is not given `publiclyAccessible: true` or `associatePublicIpAddress: true`.

**Why:** A public address puts the resource one misconfigured security group away from the internet.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(publiclyAccessible|associatePublicIpAddress)["']?$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^true$
message: This resource gets a public address; keep it in a private subnet
fix:
- replace:
    target: $V
    with: 'false'
```

## Examples

### JavaScript

#### Don't

```js
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: true });
```

#### Do

```js
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: false });
```

### JSX

#### Don't

```jsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: true });
```

#### Do

```jsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: false });
```

### TypeScript

#### Don't

```ts
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: true });
```

#### Do

```ts
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: false });
```

### TSX

#### Don't

```tsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: true });
```

#### Do

```tsx
new rds.DatabaseInstance(this, "Db", { engine, vpc, publiclyAccessible: false });
```
