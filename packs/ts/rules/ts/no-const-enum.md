---
id: ts/no-const-enum
title: "enum, not const enum"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [biome/suspicious/noConstEnum]
since: 0.5.0
---

An enum is declared without `const`.

**Why:** A `const enum` is erased at compile time, which breaks under `isolatedModules` (Babel, esbuild, SWC) and across package boundaries.

```codelaw
match:
  pattern: "const enum $NAME { $$$BODY }"
message: "Declare `$NAME` as a plain `enum`; a `const enum` breaks under isolated modules"
fix: "enum $NAME { $$$BODY }"
```

## Examples

### TypeScript

#### Don't

```ts
const enum Color {
  Red,
  Blue,
}
```

#### Do

```ts
enum Color {
  Red,
  Blue,
}
```

#### Fixed

```ts
enum Color { Red,
  Blue, }
```

### TSX

#### Don't

```tsx
const enum Color {
  Red,
  Blue,
}
```

#### Do

```tsx
enum Color {
  Red,
  Blue,
}
```

#### Fixed

```tsx
enum Color { Red,
  Blue, }
```
