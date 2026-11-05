---
id: js/no-misrefactored-shorthand-assign
title: "No compound assignment that repeats its target"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the intended expression may have been `a += b` or `a = a + b`; only the author knows which"
fix_pending: "suggest-choices"
stability: preview
ported_from: [biome/suspicious/noMisrefactoredShorthandAssign]
since: 0.5.0
---

A compound assignment does not repeat its target on the right: `a += a + b` is `a += b` (or `a = a + b`).

**Why:** It adds the target twice — almost always a half-finished refactor from `a = a + b`.

```codelaw
match:
  any:
    - pattern: $A += $A + $B
    - pattern: $A += $B + $A
    - pattern: $A -= $A - $B
    - pattern: $A *= $A * $B
    - pattern: $A *= $B * $A
    - pattern: $A /= $A / $B
    - pattern: $A %= $A % $B
    - pattern: $A **= $A ** $B
    - pattern: $A |= $A | $B
    - pattern: $A |= $B | $A
    - pattern: $A &= $A & $B
    - pattern: $A &= $B & $A
    - pattern: $A ^= $A ^ $B
    - pattern: $A ^= $B ^ $A
    - pattern: $A <<= $A << $B
    - pattern: $A >>= $A >> $B
message: "`$A` appears on both sides of a compound assignment; this applies it twice"
```

## How to fix

1. Drop the repeated target (`a += b`), or write the plain assignment you meant (`a = a + b`).

## Examples

### JavaScript

#### Don't

```js
total += total + step;
```

#### Do

```js
total += step;
```

### JSX

#### Don't

```jsx
total += total + step;
```

#### Do

```jsx
total += step;
```

### TypeScript

#### Don't

```ts
total += total + step;
```

#### Do

```ts
total += step;
```

### TSX

#### Don't

```tsx
total += total + step;
```

#### Do

```tsx
total += step;
```
