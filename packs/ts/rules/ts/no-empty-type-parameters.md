---
id: ts/no-empty-type-parameters
title: "No empty type parameter list"
languages: [typescript, tsx]
fix: auto
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
fix:
  - remove
```

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
