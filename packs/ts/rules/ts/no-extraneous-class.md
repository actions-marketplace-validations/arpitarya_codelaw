---
id: ts/no-extraneous-class
title: "No class used as a namespace"
languages: [typescript, tsx]
fix: manual
fix_reason: "the members become module-level functions, which changes every caller"
stability: preview
ported_from: [typescript-eslint/no-extraneous-class]
since: 0.5.0
---

A class is not empty and does not hold only static members.

**Why:** A class that is never instantiated is a module in disguise; plain exports are simpler.

```codelaw
match:
  kind: class_declaration
  all:
  - not:
      has: {kind: class_heritage}
  - not:
      has: {kind: decorator}
  - has:
      field: body
      any:
      - {regex: '^\{\s*\}$'}
      - all:
        - has:
            any:
            - {kind: method_definition}
            - {kind: public_field_definition}
        - not:
            has:
              any:
              - {kind: method_definition}
              - {kind: public_field_definition}
              - {kind: class_static_block}
              - {kind: abstract_method_signature}
              - {kind: method_signature}
              not: {regex: '^((public|private|protected)\s+)?static\b'}
message: This class is empty or only static; use plain functions and values
```

## How to fix

1. Export the static members as module-level functions and constants.

## Examples

### TypeScript

#### Don't

```ts
class MathUtil {
  static double(n: number) {
    return n * 2;
  }
}
```

#### Do

```ts
export function double(n: number) {
  return n * 2;
}
```

### TSX

#### Don't

```tsx
class MathUtil {
  static double(n: number) {
    return n * 2;
  }
}
```

#### Do

```tsx
export function double(n: number) {
  return n * 2;
}
```
