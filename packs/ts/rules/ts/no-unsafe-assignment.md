---
id: ts/no-unsafe-assignment
title: "No value typed `any` assigned to a typed target"
languages: [typescript, tsx]
fix: manual
fix_reason: "the type the value should have comes from the author"
fix_exception: value
stability: preview
ported_from: [typescript-eslint/no-unsafe-assignment]
tags: [types]
since: 0.4.4
---

A value typed `any` does not initialise or get assigned to a variable or property declared with
another type.

**Why:** The declared type is what every later reader trusts; an `any` assigned to it puts an
unchecked value behind a type that says it was checked.

```codelaw
match:
  any:
    - kind: variable_declarator
      has: { field: value, pattern: $V }
    - kind: public_field_definition
      has: { field: value, pattern: $V }
    - kind: assignment_expression
      has: { field: right, pattern: $V }
type:
  $V: [any, typed-target]
message: "`$V` is typed `any` but assigned to a target with a declared type"
```

## How to fix

1. Give the value a type before assigning it: annotate it, narrow it, or validate it.
2. If the target really holds anything, declare it `unknown`.

## Examples

### TypeScript

#### Don't

```ts
export const port: number = JSON.parse("8080");
```

#### Do

```ts
const raw: unknown = JSON.parse("8080");
export const port: number = typeof raw === "number" ? raw : 8080;
```

### TSX

#### Don't

```tsx
let port: number;
port = JSON.parse("8080");
```

#### Do

```tsx
let port: number;
port = Number("8080");
```

## Differences from the source

Only a target with a declared type is checked: typescript-eslint also reports `const x = value`
with `value` typed `any`, which makes `x` itself `any` (`ts/no-unsafe-member-access` and
`ts/no-unsafe-call` report what is done with it), and an `any` nested in the value's type (`any[]`
assigned to `number[]`) or in a destructuring.
