---
id: ts/no-non-null-assertion
title: "No non-null assertions"
languages: [typescript, tsx]
fix: manual
fix_reason: "handling the null — a check, optional chaining or a default — is the author's choice"
fix_pending: "suggest"
stability: preview
ported_from: [typescript-eslint/no-non-null-assertion]
since: 0.5.0
---

The non-null assertion operator `!` is not used.

**Why:** It silences the compiler's null check instead of handling the null.

```codelaw
match:
  kind: non_null_expression
  not: { inside: { kind: assignment_expression, field: left } }
  has: { pattern: $E }
message: "Handle the null instead of asserting it away"
```

## How to fix

1. Check for null, use optional chaining, or give a default with `??`.

## Examples

### TypeScript

#### Don't

```ts
const n = user!.name;
```

#### Do

```ts
const n = user?.name;
```

### TSX

#### Don't

```tsx
const n = user!.name;
```

#### Do

```tsx
const n = user?.name;
```
