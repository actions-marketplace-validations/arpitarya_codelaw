---
id: ts/no-confusing-non-null-assertion
title: "No non-null assertion next to = or =="
languages: [typescript, tsx]
fix: manual
fix_reason: "the `!` may have been meant as `!==`; the author decides which"
fix_pending: "suggest-choices"
stability: preview
ported_from: [typescript-eslint/no-confusing-non-null-assertion]
since: 0.5.0
---

A non-null assertion is not written directly before `=`, `==` or `===`.

**Why:** `a! == b` reads like `a !== b`, which means the opposite.

```codelaw
match:
  any:
    - pattern: $A! == $B
    - pattern: $A! === $B
message: "`$A! ==` reads like `!==`; drop the `!` or add parentheses"
```

## How to fix

1. Drop the `!`, or write `!==` if that was meant.

## Examples

### TypeScript

#### Don't

```ts
if (user! === other) run();
```

#### Do

```ts
if (user === other) run();
```

### TSX

#### Don't

```tsx
if (user! === other) run();
```

#### Do

```tsx
if (user === other) run();
```
