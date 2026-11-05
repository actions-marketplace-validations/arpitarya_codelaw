---
id: ts/consistent-type-definitions
title: "interface for object types"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/consistent-type-definitions]
since: 0.5.0
---

An object type is declared with `interface`, not `type X = { … }`.

**Why:** Interfaces give clearer errors, can be extended and merged, and are the form the TypeScript handbook recommends for object shapes.

```codelaw
match:
  kind: type_alias_declaration
  all:
  - has: {field: name, pattern: $N}
  - has: {field: value, kind: object_type, pattern: $B}
  - not:
      has: {field: type_parameters, regex: .}
  - not:
      inside: {kind: export_statement, regex: ^export\s+default}
message: Declare `$N` as an interface
fix: interface $N $B
```

## Examples

### TypeScript

#### Don't

```ts
type Point = { x: number; y: number };
```

#### Do

```ts
interface Point { x: number; y: number }
```

#### Fixed

```ts
interface Point { x: number; y: number }
```

### TSX

#### Don't

```tsx
type Point = { x: number; y: number };
```

#### Do

```tsx
interface Point { x: number; y: number }
```

#### Fixed

```tsx
interface Point { x: number; y: number }
```

## Differences from the source

A type alias with type parameters is left alone.
