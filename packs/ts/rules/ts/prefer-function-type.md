---
id: ts/prefer-function-type
title: "A function type, not an interface with one call signature"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/prefer-function-type]
since: 0.5.0
---

An interface whose only member is a call signature is written as a function type alias.

**Why:** `type F = (x: T) => R` is the plainer way to name a function type.

```codelaw
match:
  kind: interface_declaration
  all:
  - not:
      has: {kind: extends_type_clause}
  - not:
      has: {field: type_parameters, regex: .}
  - has: {field: name, pattern: $N}
  - has:
      field: body
      has:
        kind: call_signature
        all:
        - has: {field: parameters, pattern: $P}
        - has:
            field: return_type
            has: {pattern: $R, regex: .}
      not:
        has: {nthChild: 2, regex: '[\s\S]'}
message: Write `$N` as a function type
fix: type $N = $P => $R;
```

## Examples

### TypeScript

#### Don't

```ts
interface Handler {
  (e: Event): void;
}
```

#### Do

```ts
type Handler = (e: Event) => void;
```

#### Fixed

```ts
type Handler = (e: Event) => void;
```

### TSX

#### Don't

```tsx
interface Handler {
  (e: Event): void;
}
```

#### Do

```tsx
type Handler = (e: Event) => void;
```

#### Fixed

```tsx
type Handler = (e: Event) => void;
```
