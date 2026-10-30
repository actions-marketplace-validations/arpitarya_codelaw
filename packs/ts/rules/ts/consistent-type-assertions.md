---
id: ts/consistent-type-assertions
title: "as, not angle-bracket assertions"
languages: [typescript]
fix: auto
stability: preview
ported_from: [typescript-eslint/consistent-type-assertions]
since: 0.5.0
---

A type assertion is written `x as T`, not `<T>x`.

**Why:** The angle-bracket form cannot be used in TSX and reads like a generic.

```codelaw
match:
  kind: type_assertion
  all:
  - has:
      kind: type_arguments
      has: {pattern: $T}
  - has:
      not: {kind: type_arguments}
      pattern: $E
      regex: .
message: Write the assertion as `as $T`
fix: ($E as $T)
```

## Examples

### TypeScript

#### Don't

```ts
const n = <number>value;
```

#### Do

```ts
const n = (value as number);
```

#### Fixed

```ts
const n = (value as number);
```
