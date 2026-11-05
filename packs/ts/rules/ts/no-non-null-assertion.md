---
id: ts/no-non-null-assertion
title: "No non-null assertions"
languages: [typescript, tsx]
fix: suggest
fix_reason: "optional chaining turns a null into `undefined` instead of failing; the reviewer confirms that is the handling wanted (offered where the `!` is followed by `.name`)"
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
  any:
    - all:
        - pattern: $W
        - inside: { kind: member_expression, stopBy: neighbor }
    - kind: non_null_expression
message: "Handle the null instead of asserting it away"
fix:
  - replace: { target: $W, with: "$E?" }
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
