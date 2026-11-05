---
id: js/file-permissions
title: "No world-writable file modes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right mode depends on who must read the file"
stability: preview
ported_from: [eslint-plugin-sonarjs/file-permissions]
since: 0.5.0
---

A file-mode argument does not give write permission to "others" (`0o777`, `0o666`, …).

**Why:** Any local user or process could change the file.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)(chmod|chmodSync|fchmod|fchmodSync|lchmod|lchmodSync|mkdir|mkdirSync|writeFile|writeFileSync|open|openSync)$
  all:
  - has:
      field: arguments
      has:
        kind: number
        regex: ^0o?[0-7]?[0-7][0-7][2367]$
message: This mode lets every user on the machine write the file
```

## How to fix

1. Remove write (and, if possible, read) permission for others: `0o750` for directories, `0o640` for files.

## Examples

### JavaScript

#### Don't

```js
fs.chmodSync(path, 0o777);
```

#### Do

```js
fs.chmodSync(path, 0o750);
```

### JSX

#### Don't

```jsx
fs.chmodSync(path, 0o777);
```

#### Do

```jsx
fs.chmodSync(path, 0o750);
```

### TypeScript

#### Don't

```ts
fs.chmodSync(path, 0o777);
```

#### Do

```ts
fs.chmodSync(path, 0o750);
```

### TSX

#### Don't

```tsx
fs.chmodSync(path, 0o777);
```

#### Do

```tsx
fs.chmodSync(path, 0o750);
```
