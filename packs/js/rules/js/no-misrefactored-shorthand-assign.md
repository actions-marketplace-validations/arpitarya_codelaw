---
id: js/no-misrefactored-shorthand-assign
title: "No compound assignment that repeats its target"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the target is repeated on the right, so either the compound form or the long form was meant: both are offered and the reviewer picks"
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
      has: { field: right, pattern: $R }
    - pattern: $A += $B + $A
      has: { field: right, pattern: $R }
    - pattern: $A -= $A - $B
      has: { field: right, pattern: $R }
    - pattern: $A *= $A * $B
      has: { field: right, pattern: $R }
    - pattern: $A *= $B * $A
      has: { field: right, pattern: $R }
    - pattern: $A /= $A / $B
      has: { field: right, pattern: $R }
    - pattern: $A %= $A % $B
      has: { field: right, pattern: $R }
    - pattern: $A **= $A ** $B
      has: { field: right, pattern: $R }
    - pattern: $A |= $A | $B
      has: { field: right, pattern: $R }
    - pattern: $A |= $B | $A
      has: { field: right, pattern: $R }
    - pattern: $A &= $A & $B
      has: { field: right, pattern: $R }
    - pattern: $A &= $B & $A
      has: { field: right, pattern: $R }
    - pattern: $A ^= $A ^ $B
      has: { field: right, pattern: $R }
    - pattern: $A ^= $B ^ $A
      has: { field: right, pattern: $R }
    - pattern: $A <<= $A << $B
      has: { field: right, pattern: $R }
    - pattern: $A >>= $A >> $B
      has: { field: right, pattern: $R }
message: "`$A` appears on both sides of a compound assignment; this applies it twice"
fix:
  choices:
    - label: compound assignment
      default: true
      fix:
        - replace: { target: $R, with: "$B" }
    - label: long form
      fix: "$A = $R"
```

## How to fix

1. Drop the repeated target (`a += b`), or write the plain assignment you meant (`a = a + b`).

## Examples

### JavaScript

#### Don't

```js
total += total + step;
```

#### Fixed (compound assignment)

```js
total += step;
```

#### Fixed (long form)

```js
total = total + step;
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

#### Fixed (compound assignment)

```jsx
total += step;
```

#### Fixed (long form)

```jsx
total = total + step;
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

#### Fixed (compound assignment)

```ts
total += step;
```

#### Fixed (long form)

```ts
total = total + step;
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

#### Fixed (compound assignment)

```tsx
total += step;
```

#### Fixed (long form)

```tsx
total = total + step;
```

#### Do

```tsx
total += step;
```
