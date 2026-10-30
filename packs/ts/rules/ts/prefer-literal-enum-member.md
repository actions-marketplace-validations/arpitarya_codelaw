---
id: ts/prefer-literal-enum-member
title: "Enum members are literals"
languages: [typescript, tsx]
fix: manual
fix_reason: "the literal value is the author's"
stability: preview
ported_from: [typescript-eslint/prefer-literal-enum-member]
since: 0.5.0
---

An enum member's value is a literal (a string, a number or a negative number), not a computed expression.

**Why:** Computed members make an enum's values depend on evaluation order and other code.

```codelaw
match:
  kind: enum_assignment
  has:
    field: value
    not: { any: [{kind: string}, {kind: number}, {kind: template_string, not: { has: { kind: template_substitution } }}, {kind: unary_expression, regex: '^[-+]\s*\d'}] }
message: "Use a literal value for this enum member"
```

## How to fix

1. Write the value as a literal.

## Examples

### TypeScript

#### Don't

```ts
enum Size {
  Big = base * 2,
}
```

#### Do

```ts
enum Size {
  Big = 20,
}
```

### TSX

#### Don't

```tsx
enum Size {
  Big = base * 2,
}
```

#### Do

```tsx
enum Size {
  Big = 20,
}
```
