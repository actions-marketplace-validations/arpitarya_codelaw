---
id: ts/no-empty-type-parameters
title: "No empty type parameter list"
languages: [typescript, tsx]
fix: manual
fix_reason: "either a parameter is missing or the brackets should go; the author knows which"
stability: preview
ported_from: [biome/complexity/noEmptyTypeParameters]
since: 0.5.0
---

A type parameter list `<>` is not left empty.

**Why:** An empty list declares nothing and is a leftover of an edit.

```codelaw
match:
  kind: type_parameters
  regex: '^<\s*>$'
message: "Remove the empty type parameter list, or declare the parameter it was meant for"
```

## How to fix

1. Delete `<>`, or add the type parameter it was meant to hold.

## Examples

### TypeScript

#### Don't

```ts
interface Box<> {
  value: string;
}
```

#### Do

```ts
interface Box {
  value: string;
}
```

### TSX

#### Don't

```tsx
interface Box<> {
  value: string;
}
```

#### Do

```tsx
interface Box {
  value: string;
}
```
