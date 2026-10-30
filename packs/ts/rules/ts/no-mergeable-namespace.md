---
id: ts/no-mergeable-namespace
title: "A namespace is declared once per file"
languages: [typescript, tsx]
fix: manual
fix_reason: "restructure: the two bodies are merged into one, and their order and exports are the author's"
stability: preview
ported_from: [tslint/no-mergeable-namespace]
since: 0.5.0
---

Two `namespace` (or `module`) declarations of the same name in one file are written as one.

**Why:** TypeScript merges them silently; a reader who finds one rarely looks for the other.

```codelaw
utils:
  ns:
    any:
      - { kind: internal_module, has: { field: name, kind: identifier, pattern: $N } }
      - { kind: module, has: { field: name, kind: identifier, pattern: $N } }
  holds-ns:
    any:
      - matches: ns
      - has: { matches: ns }
      - has: { has: { matches: ns } }
match:
  matches: ns
  inside:
    stopBy: end
    matches: holds-ns
    follows: { stopBy: end, matches: holds-ns }
message: "Namespace `$N` is already declared in this file; merge the two"
```

## How to fix

1. Move this body into the first `namespace $N` and delete this one.

## Examples

### TypeScript

#### Don't

```ts
namespace Shapes { export const square = 1; }
namespace Shapes { export const circle = 2; }
```

#### Do

```ts
namespace Shapes {
  export const square = 1;
  export const circle = 2;
}
```

### TSX

#### Don't

```tsx
namespace Shapes { export const square = 1; }
namespace Shapes { export const circle = 2; }
```

#### Do

```tsx
namespace Shapes {
  export const square = 1;
  export const circle = 2;
}
```
