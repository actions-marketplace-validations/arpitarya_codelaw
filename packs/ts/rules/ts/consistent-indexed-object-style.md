---
id: ts/consistent-indexed-object-style
title: "Record rather than an index signature"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/consistent-indexed-object-style]
since: 0.5.0
---

A type made only of one index signature is written `Record<K, V>`.

**Why:** `Record<string, T>` says the same in a form readers recognise at once.

```codelaw
match:
  kind: object_type
  has:
    kind: index_signature
    all:
    - has: {field: index_type, pattern: $KT}
    - has:
        kind: type_annotation
        has: {pattern: $V, regex: .}
  not:
    has: {nthChild: 2, regex: '[\s\S]'}
message: Use `Record<$KT, $V>` instead of an index signature
fix: Record<$KT, $V>
```

## Examples

### TypeScript

#### Don't

```ts
type Counts = { [name: string]: number };
```

#### Do

```ts
type Counts = Record<string, number>;
```

#### Fixed

```ts
type Counts = Record<string, number>;
```

### TSX

#### Don't

```tsx
type Counts = { [name: string]: number };
```

#### Do

```tsx
type Counts = Record<string, number>;
```

#### Fixed

```tsx
type Counts = Record<string, number>;
```
