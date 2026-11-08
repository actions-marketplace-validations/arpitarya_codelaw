---
id: ts/explicit-function-return-type
title: "Functions declare their return type"
languages: [typescript, tsx]
fix: manual
fix_reason: "the return type is the author's to write"
fix_pending: "suggest; needs type-info (W-105)"
stability: preview
ported_from: [typescript-eslint/explicit-function-return-type]
since: 0.5.0
---

A function declaration or method declares its return type.

**Why:** An explicit return type documents the contract and catches a wrong return at the function, not at every caller.

```codelaw
match:
  all:
  - any:
    - {kind: function_declaration}
    - {kind: method_definition}
    - {kind: generator_function_declaration}
  - not:
      has: {field: return_type, regex: .}
  - not:
      has: {field: name, regex: ^(constructor|set)$}
  - not: {regex: ^(get|set|static\s+(get|set))\s}
message: Declare this function's return type
```

## How to fix

1. Add `: ReturnType` after the parameter list.

## Examples

### TypeScript

#### Don't

```ts
function total(xs: number[]) {
  return xs.length;
}
```

#### Do

```ts
function total(xs: number[]): number {
  return xs.length;
}
```

### TSX

#### Don't

```tsx
function total(xs: number[]) {
  return xs.length;
}
```

#### Do

```tsx
function total(xs: number[]): number {
  return xs.length;
}
```

## Differences from the source

Function and arrow expressions are not checked; the source checks them unless they are typed by their context, which needs type information.
