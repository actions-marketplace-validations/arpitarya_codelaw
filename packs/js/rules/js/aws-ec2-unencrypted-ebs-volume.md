---
id: js/aws-ec2-unencrypted-ebs-volume
title: "EBS volumes are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "encrypting an existing volume means a new volume; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-ec2-unencrypted-ebs-volume]
since: 0.5.0
---

An AWS CDK EC2 `Volume` is not created with `encrypted: false`.

**Why:** An unencrypted volume or its snapshots expose the data.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(encrypted)["']?$
  inside:
    stopBy: end
    kind: new_expression
    has:
      field: constructor
      regex: (^|\.)Volume$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: This EBS volume is not encrypted at rest
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: false });
```

#### Do

```js
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: true });
```

### JSX

#### Don't

```jsx
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: false });
```

#### Do

```jsx
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: true });
```

### TypeScript

#### Don't

```ts
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: false });
```

#### Do

```ts
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: true });
```

### TSX

#### Don't

```tsx
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: false });
```

#### Do

```tsx
new ec2.Volume(this, "Disk", { availabilityZone, size, encrypted: true });
```
