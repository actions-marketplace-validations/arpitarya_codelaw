---
id: ts/prefer-enum-initializers
title: "Enum members have initialisers"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the suggested value fixes today's order; review it before relying on it"
stability: preview
ported_from: [typescript-eslint/prefer-enum-initializers]
since: 0.5.0
---

Each enum member is given its value explicitly.

**Why:** Implicit values change when members are reordered or inserted, which silently changes stored data.

```codelaw
match:
  kind: property_identifier
  inside: { kind: enum_body }
  pattern: $M
message: "Give `$M` an explicit value"
fix: "$M = '$M'"
```

## Examples

### TypeScript

#### Don't

```ts
enum Status {
  Open,
}
```

#### Do

```ts
enum Status {
  Open = 'Open',
}
```

### TSX

#### Don't

```tsx
enum Status {
  Open,
}
```

#### Do

```tsx
enum Status {
  Open = 'Open',
}
```
