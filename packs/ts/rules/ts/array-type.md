---
id: ts/array-type
title: "T[] rather than Array<T>"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/array-type]
since: 0.5.0
---

A simple array type is written `T[]`, not `Array<T>` (and `readonly T[]`, not `ReadonlyArray<T>`).

**Why:** One spelling for one type reads faster; `T[]` is the one TypeScript prints.

```codelaw
match:
  kind: generic_type
  all:
  - has: {field: name, regex: ^Array$}
  - has:
      field: type_arguments
      has:
        any:
        - {kind: type_identifier}
        - {kind: predefined_type}
        - {kind: array_type}
        - {kind: generic_type}
        pattern: $T
      not:
        has: {nthChild: 2, regex: '[\s\S]'}
message: Write the array type with `[]`
fix: $T[]
```

## Examples

### TypeScript

#### Don't

```ts
let names: Array<string>;
```

#### Do

```ts
let names: string[];
```

#### Fixed

```ts
let names: string[];
```

### TSX

#### Don't

```tsx
let names: Array<string>;
```

#### Do

```tsx
let names: string[];
```

#### Fixed

```tsx
let names: string[];
```

## Differences from the source

Only `Array<T>` whose element type needs no parentheses (a name, a builtin type, another array or generic) is reported; `ReadonlyArray<T>` and union elements are left for now.
