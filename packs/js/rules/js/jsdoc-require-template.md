---
id: js/jsdoc-require-template
title: "Type parameters are documented with @template"
languages: [typescript, tsx]
fix: manual
fix_reason: "each type parameter needs its own @template line with the author's description"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-template]
since: 0.5.0
---

A JSDoc block on a generic function, class, interface or type alias has `@template` tags. Ported in part: a block with no `@template` at all is reported; one that documents some type parameters and misses others is not. JavaScript `@typedef` generics are not checked.

**Why:** A generic's type parameters are part of its contract, and readers look for them next to `@param`.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - not:
      regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@template\b
  - not:
      regex: (?i)@(?:inheritdoc|override|type|file|fileoverview|overview|license|module|copyright)\b
  precedes:
    any:
    - all: &id001
      - any:
        - kind: function_declaration
        - kind: class_declaration
        - kind: abstract_class_declaration
        - kind: interface_declaration
        - kind: type_alias_declaration
        - kind: function_signature
      - has:
          field: type_parameters
          regex: .
    - kind: export_statement
      has:
        field: declaration
        all: *id001
message: This declaration is generic; add a `@template` tag for each type parameter
```

## How to fix

1. Add `@template T description` for each type parameter, in order.

## Examples

### TypeScript

#### Don't

```ts
/**
 * Wraps a value.
 * @param value The value.
 * @returns The box.
 */
export function box<T>(value: T): T[] {
  return [value];
}
```

#### Do

```ts
/**
 * Wraps a value.
 * @template T The value's type.
 * @param value The value.
 * @returns The box.
 */
export function box<T>(value: T): T[] {
  return [value];
}
```

### TSX

#### Don't

```tsx
/**
 * Wraps a value.
 * @param value The value.
 * @returns The box.
 */
export function box<T>(value: T): T[] {
  return [value];
}
```

#### Do

```tsx
/**
 * Wraps a value.
 * @template T The value's type.
 * @param value The value.
 * @returns The box.
 */
export function box<T>(value: T): T[] {
  return [value];
}
```
