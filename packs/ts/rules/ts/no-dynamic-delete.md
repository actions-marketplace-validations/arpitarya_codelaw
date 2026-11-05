---
id: ts/no-dynamic-delete
title: "No delete with a computed key"
languages: [typescript, tsx]
fix: manual
fix_reason: "a `Map` or a rest-destructure changes the data structure"
fix_exception: restructure
stability: preview
ported_from: [typescript-eslint/no-dynamic-delete]
since: 0.5.0
---

`delete` is not used with a computed key (`delete obj[key]`).

**Why:** Objects used as dictionaries are better as `Map`; deleting dynamic keys is slow and defeats typing.

```codelaw
match:
  kind: unary_expression
  regex: '^delete\b'
  has: { kind: subscript_expression, has: { field: index, not: { any: [{kind: string}, {kind: number}] } } }
message: "Do not `delete` a computed key; use a `Map`"
```

## How to fix

1. Use a `Map` and its `delete`, or build a new object without the key.

## Examples

### TypeScript

#### Don't

```ts
delete cache[key];
```

#### Do

```ts
cache.delete(key);
```

### TSX

#### Don't

```tsx
delete cache[key];
```

#### Do

```tsx
cache.delete(key);
```
