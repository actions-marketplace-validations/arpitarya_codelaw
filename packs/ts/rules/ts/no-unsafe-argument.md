---
id: ts/no-unsafe-argument
title: "No argument typed `any` for a typed parameter"
languages: [typescript, tsx]
fix: manual
fix_reason: "the type the value should have comes from the author"
stability: preview
ported_from: [typescript-eslint/no-unsafe-argument]
tags: [types]
since: 0.4.4
---

A value typed `any` is not passed to a parameter declared with another type.

**Why:** The parameter's type is a promise the caller makes; an `any` argument breaks it unchecked,
and the function receives whatever the value turns out to be.

```codelaw
match:
  pattern: $A
  inside: { kind: arguments, stopBy: neighbor }
type:
  $A: [any, typed-target]
message: "`$A` is typed `any` but passed to a parameter with a declared type"
```

## How to fix

1. Give the value a type before passing it: annotate it, narrow it, or validate it.
2. If the parameter really accepts anything, declare it `unknown`.

## Examples

### TypeScript

#### Don't

```ts
declare const loose: any;
function double(n: number): number {
  return n * 2;
}
double(loose);
```

#### Do

```ts
declare const loose: unknown;
function double(n: number): number {
  return n * 2;
}
double(typeof loose === "number" ? loose : 0);
```

### TSX

#### Don't

```tsx
declare const loose: any;
function double(n: number): number {
  return n * 2;
}
double(loose);
```

#### Do

```tsx
declare const loose: any;
function show(value: unknown): void {}
show(loose);
```

## Differences from the source

A spread argument (`f(...values)`) and an `any` nested in an argument's type (`any[]` passed for
`number[]`) are not checked.
