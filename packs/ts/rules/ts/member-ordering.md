---
id: ts/member-ordering
title: "Class members are in a set order"
languages: [typescript, tsx]
fix: manual
fix_reason: "moving members carries their comments, and a field's initialiser may read one declared before it, so the order is the author's to change"
fix_exception: restructure
stability: preview
ported_from: [typescript-eslint/member-ordering]
since: 0.5.0
---

Members come in this order: index signatures, static fields, instance fields, static blocks, the constructor, accessors, static methods, instance methods; within each, public, protected, private, then `#private`.

**Why:** A reader finds a member where they expect it.

```codelaw
native: ts/member-ordering
```

## How to fix

1. Move the member up to its group.

## Examples

### TypeScript

#### Don't

```ts
class A {
  run(): void {}
  name = "a";
}
```

#### Do

```ts
class A {
  name = "a";
  run(): void {}
}
```

### TSX

#### Don't

```tsx
class A {
  run(): void {}
  name = "a";
}
```

#### Do

```tsx
class A {
  name = "a";
  run(): void {}
}
```
