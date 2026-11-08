---
id: ts/no-implicit-any-let
title: "A let without a value has a type"
languages: [typescript, tsx]
fix: manual
fix_reason: "the variable's type is the author's to write"
fix_pending: "auto-subset; needs type-info (W-105)"
stability: preview
ported_from: [biome/suspicious/noImplicitAnyLet]
since: 0.5.0
---

A `let` declared without a value carries a type annotation.

**Why:** Without one its type is `any` until TypeScript's control-flow analysis guesses, and type errors slip past.

```codelaw
match:
  kind: variable_declarator
  all:
  - not:
      has: {field: type, regex: .}
  - not:
      has: {field: value, regex: .}
  - inside:
      kind: lexical_declaration
      regex: ^let\b
      not:
        inside: {kind: ambient_declaration}
message: Give this `let` a type annotation or an initial value
```

## How to fix

1. Add the type (`let count: number;`) or initialise the variable.

## Examples

### TypeScript

#### Don't

```ts
let count;
count = 1;
```

#### Do

```ts
let count: number;
count = 1;
```

### TSX

#### Don't

```tsx
let count;
count = 1;
```

#### Do

```tsx
let count: number;
count = 1;
```
