---
id: ts/switch-exhaustiveness-check
title: "A `switch` on a union handles every member"
languages: [typescript, tsx]
fix: manual
fix_reason: "what each missing member should do is the author's decision"
stability: preview
ported_from: [typescript-eslint/switch-exhaustiveness-check]
tags: [types]
since: 0.4.4
---

A `switch` whose value is a union of literal types (string, number or boolean literals, enum
members, `null`, `undefined`) has a `case` for every member, or a `default`.

**Why:** When a member is added to the union, a `switch` that silently skips it lets the new value
fall through to whatever comes after; with every member handled, TypeScript points at each `switch`
that needs the new case.

```codelaw
match:
  kind: switch_statement
type:
  $$MATCH: non-exhaustive
message: "This `switch` has no `case` for some members of its value's type, and no `default`"
```

## How to fix

1. Add a `case` for each member of the value's type the `switch` leaves out (hover the value, or
   read its type, to list them).
2. If the members left out really share one behaviour, add a `default` instead.

## Examples

### TypeScript

#### Don't

```ts
type Shape = "circle" | "square";
export function sides(s: Shape): number {
  switch (s) {
    case "circle":
      return 0;
  }
  return -1;
}
```

#### Do

```ts
type Shape = "circle" | "square";
export function sides(s: Shape): number {
  switch (s) {
    case "circle":
      return 0;
    case "square":
      return 4;
  }
}
```

### TSX

#### Don't

```tsx
type Shape = "circle" | "square";
export function sides(s: Shape): number {
  switch (s) {
    case "circle":
      return 0;
  }
  return -1;
}
```

#### Do

```tsx
type Shape = "circle" | "square";
export function sides(s: Shape): number {
  switch (s) {
    case "circle":
      return 0;
    default:
      return 4;
  }
}
```

## Differences from the source

A `switch` with a `default` is never reported (typescript-eslint's
`considerDefaultExhaustiveForUnions` set to `true`), and none of the rule's options are offered: a
value whose type is not made only of literal types (`string`, an object union) is not checked
(`requireDefaultForNonUnion`), and a `default` in an exhaustive `switch` is allowed
(`allowDefaultCaseForExhaustiveSwitch`). typescript-eslint's suggestion that writes the missing cases
is not offered: SR-TYPES's questions answer whether members are missing, not which.
