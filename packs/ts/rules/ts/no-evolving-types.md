---
id: ts/no-evolving-types
title: "No variable that evolves into another type"
languages: [typescript, tsx]
fix: manual
fix_reason: "the type to write is the author's knowledge of what the variable will hold"
fix_exception: restructure
stability: preview
ported_from: [biome/suspicious/noEvolvingTypes]
since: 0.5.1
---

A variable with no type annotation that starts as `null`, `undefined` (`let` or `var`) or an empty array (`let`, `var` or `const`) is reported: without `noImplicitAny` narrowing it grows into whatever is assigned to it. A declaration with no initializer is `ts/no-implicit-any-let`'s. Behaviour of Biome's `noEvolvingTypes` as it can be read without type information.

**Why:** Such a variable has no type of its own: the compiler infers `any` or a union from later writes, so a wrong write is not caught where it is made.

```codelaw
match:
  kind: variable_declarator
  all:
    - has: { field: name, kind: identifier }
    - not: { has: { field: type, kind: type_annotation } }
    - any:
        - has: { field: value, kind: array, regex: '^\[\s*\]$' }
        - all:
            - has: { field: value, any: [{ kind: "null" }, { kind: identifier, regex: '^undefined$' }] }
            - any:
                - inside: { kind: lexical_declaration, regex: '^let\b' }
                - inside: { kind: variable_declaration }
message: "Give this variable a type annotation; it would evolve with each assignment"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### TypeScript

#### Don't

```ts
let current = null;
current = 3;
```

#### Do

```ts
let current: number | null = null;
current = 3;
```

### TSX

#### Don't

```tsx
let current = null;
current = 3;
```

#### Do

```tsx
let current: number | null = null;
current = 3;
```
