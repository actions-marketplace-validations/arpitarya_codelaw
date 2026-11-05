---
id: ts/no-non-null-asserted-nullish-coalescing
title: "No ! before ??"
languages: [typescript, tsx]
fix: suggest
fix_reason: "removing `!` is right if the left side may be null, which is the point of `??`"
stability: preview
ported_from: [typescript-eslint/no-non-null-asserted-nullish-coalescing]
since: 0.5.0
---

The left side of `??` is not non-null asserted.

**Why:** `a! ?? b` says `a` is never null, so `b` can never be used — one of the two is wrong.

```codelaw
match:
  pattern: $A! ?? $B
message: "Remove the `!`: `??` exists because `$A` may be null"
fix: "$A ?? $B"
```

## Examples

### TypeScript

#### Don't

```ts
const name = user.name! ?? 'anon';
```

#### Do

```ts
const name = user.name ?? 'anon';
```

### TSX

#### Don't

```tsx
const name = user.name! ?? 'anon';
```

#### Do

```tsx
const name = user.name ?? 'anon';
```
