---
id: ts/no-duplicate-enum-values
title: "No two enum members share a value"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the choices are an explicit alias of the first member with that value, keeping both equal, or the next unused number, making them differ; which one is meant is the reviewer's call"
fix_exception: value
stability: preview
ported_from: [typescript-eslint/no-duplicate-enum-values]
since: 0.1.0
---

Each member of an enum given a literal value has a different value.

**Why:** Two members with one value are indistinguishable at run time, which is usually a copy-paste
slip.

```codelaw
native: ts/no-duplicate-enum-values
```

## How to fix

1. Give each member its own value.

## Examples

### TypeScript

#### Don't

```ts
enum Color {
  Red = 1,
  Green = 1,
}
```

#### Fixed (alias)

```ts
enum Color {
  Red = 1,
  Green = Red,
}
```

#### Fixed (next value)

```ts
enum Color {
  Red = 1,
  Green = 2,
}
```

#### Do

```ts
enum Color {
  Red = 1,
  Green = 2,
}
```

### TSX

#### Don't

```tsx
enum Color {
  Red = 1,
  Green = 1,
}
```

#### Fixed (alias)

```tsx
enum Color {
  Red = 1,
  Green = Red,
}
```

#### Fixed (next value)

```tsx
enum Color {
  Red = 1,
  Green = 2,
}
```

#### Do

```tsx
enum Color {
  Red = 1,
  Green = 2,
}
```
