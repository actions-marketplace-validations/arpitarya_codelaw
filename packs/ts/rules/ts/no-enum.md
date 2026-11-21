---
id: ts/no-enum
title: "No `enum`"
languages: [typescript, tsx]
fix: manual
fix_reason: "an enum becomes an object `as const` and a union type, and its users change with it"
fix_exception: restructure
stability: preview
ported_from: [biome/style/noEnum]
since: 0.5.1
---

An `enum` (a `const enum` too) is not declared; an object with `as const` and a union of its values does the same job. An `enum` inside a `declare` block is left alone. Behaviour of Biome's `noEnum`.

**Why:** An enum is TypeScript's own syntax with a runtime shape: it emits code, cannot be erased, and does not work under `erasableSyntaxOnly` or in a runtime that strips types.

```codelaw
match:
  kind: enum_declaration
  not: { inside: { stopBy: end, kind: ambient_declaration } }
message: "Use an object `as const` and a union type instead of an `enum`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### TypeScript

#### Don't

```ts
enum Direction {
  Up,
  Down,
}
```

#### Do

```ts
const Direction = {
  Up: "Up",
  Down: "Down",
} as const;
type Direction = (typeof Direction)[keyof typeof Direction];
```

### TSX

#### Don't

```tsx
enum Direction {
  Up,
  Down,
}
```

#### Do

```tsx
const Direction = {
  Up: "Up",
  Down: "Down",
} as const;
type Direction = (typeof Direction)[keyof typeof Direction];
```
