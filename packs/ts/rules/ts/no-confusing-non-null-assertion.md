---
id: ts/no-confusing-non-null-assertion
title: "No non-null assertion next to = or =="
languages: [typescript, tsx]
fix: suggest
fix_reason: "the `!` may have been meant as the start of `!=`/`!==` or as a stray assertion; the reviewer picks which"
stability: preview
ported_from: [typescript-eslint/no-confusing-non-null-assertion]
since: 0.5.0
---

A non-null assertion is not written directly before `=`, `==` or `===`.

**Why:** `a! == b` reads like `a !== b`, which means the opposite.

```codelaw
match:
  any:
    - all:
        - pattern: $A1! == $B1
        - pattern: $W1
    - all:
        - pattern: $A2! === $B2
        - pattern: $W2
message: "`$A! ==` reads like `!==`; drop the `!` or add parentheses"
fix:
  choices:
    - label: drop the !
      default: true
      fix:
        - replace: { target: $W1, with: "$A1 == $B1" }
        - replace: { target: $W2, with: "$A2 === $B2" }
    - label: not equal
      fix:
        - replace: { target: $W1, with: "$A1 != $B1" }
        - replace: { target: $W2, with: "$A2 !== $B2" }
```

## How to fix

1. Drop the `!`, or write `!==` if that was meant.

## Examples

### TypeScript

#### Don't

```ts
if (user! === other) run();
```

#### Fixed (drop the !)

```ts
if (user === other) run();
```

#### Fixed (not equal)

```ts
if (user !== other) run();
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

#### Fixed (drop the !)

```tsx
if (user === other) run();
```

#### Fixed (not equal)

```tsx
if (user !== other) run();
```

#### Do

```tsx
if (user === other) run();
```
