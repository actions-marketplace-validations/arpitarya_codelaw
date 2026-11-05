---
id: ts/method-signature-style
title: "Property signatures for methods"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/method-signature-style]
since: 0.5.0
---

A method in an interface or type literal is written as a property with a function type (`m: () => void`), not a method signature (`m(): void`).

**Why:** Property signatures are checked strictly (contravariantly) under `strictFunctionTypes`; method signatures are not.

```codelaw
match:
  kind: method_signature
  all:
  - has: {field: name, pattern: $N}
  - has: {field: parameters, pattern: $P}
  - has:
      field: return_type
      has: {pattern: $R, regex: .}
  - not:
      has: {field: type_parameters, regex: .}
  - not: {regex: ^(get|set)\s}
  - not: {regex: '\?\s*\('}
message: Write `$N` as a property with a function type
fix: '$N: $P => $R'
```

## Examples

### TypeScript

#### Don't

```ts
interface Logger {
  log(msg: string): void;
}
```

#### Do

```ts
interface Logger {
  log: (msg: string) => void;
}
```

#### Fixed

```ts
interface Logger {
  log: (msg: string) => void;
}
```

### TSX

#### Don't

```tsx
interface Logger {
  log(msg: string): void;
}
```

#### Do

```tsx
interface Logger {
  log: (msg: string) => void;
}
```

#### Fixed

```tsx
interface Logger {
  log: (msg: string) => void;
}
```

## Differences from the source

Generic and optional methods are left for a manual rewrite.
