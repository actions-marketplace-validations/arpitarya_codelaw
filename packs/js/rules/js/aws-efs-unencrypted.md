---
id: js/aws-efs-unencrypted
title: "EFS file systems are encrypted"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "encryption cannot be turned on for an existing file system; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-efs-unencrypted]
since: 0.5.0
---

An AWS CDK EFS `FileSystem` is not created with `encrypted: false`.

**Why:** Unencrypted storage exposes data to anyone who gets the disks or snapshots.

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
      regex: (^|\.)FileSystem$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: This file system is not encrypted at rest
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
new efs.FileSystem(this, "Fs", { vpc, encrypted: false });
```

#### Do

```js
new efs.FileSystem(this, "Fs", { vpc, encrypted: true });
```

### JSX

#### Don't

```jsx
new efs.FileSystem(this, "Fs", { vpc, encrypted: false });
```

#### Do

```jsx
new efs.FileSystem(this, "Fs", { vpc, encrypted: true });
```

### TypeScript

#### Don't

```ts
new efs.FileSystem(this, "Fs", { vpc, encrypted: false });
```

#### Do

```ts
new efs.FileSystem(this, "Fs", { vpc, encrypted: true });
```

### TSX

#### Don't

```tsx
new efs.FileSystem(this, "Fs", { vpc, encrypted: false });
```

#### Do

```tsx
new efs.FileSystem(this, "Fs", { vpc, encrypted: true });
```
