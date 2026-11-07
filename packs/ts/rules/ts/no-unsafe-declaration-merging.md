---
id: ts/no-unsafe-declaration-merging
title: "No interface that merges with a class of the same name"
languages: [typescript, tsx]
fix: auto
fix_reason: "auto when the interface holds only property signatures, has no `extends` or type parameters, is not exported and names nothing the class declares: they become `declare` fields of the class, which emit nothing, so the type and the runtime are unchanged; any other interface is manual"
fix_exception: restructure
stability: preview
ported_from: [typescript-eslint/no-unsafe-declaration-merging]
since: 0.1.0
---

An interface doesn't share its name with a class in the same scope.

**Why:** The merged interface adds members to the class's type that the class never initialises, and
the compiler can't tell.

```codelaw
native: ts/no-unsafe-declaration-merging
```

## How to fix

1. Move the interface's members into the class, or rename the interface.

## Examples

### TypeScript

#### Don't

```ts
interface Point {
  z: number;
}
class Point {
  x = 0;
}
```

#### Fixed

```ts
class Point {
  declare z: number;
  x = 0;
}
```

#### Do

```ts
interface PointLike {
  z: number;
}
class Point {
  x = 0;
}
```

### TSX

#### Don't

```tsx
interface Point {
  z: number;
}
class Point {
  x = 0;
}
```

#### Fixed

```tsx
class Point {
  declare z: number;
  x = 0;
}
```

#### Do

```tsx
interface PointLike {
  z: number;
}
class Point {
  x = 0;
}
```
