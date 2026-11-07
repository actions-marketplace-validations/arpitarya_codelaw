---
id: js/consistent-tuple-labels
title: "Label every element of a tuple type, or none"
languages: [typescript, tsx]
fix: manual
fix_reason: "the label names what a position means, which only the author knows"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-tuple-labels]
since: 0.5.0
---

A TypeScript tuple type that labels some of its elements labels all of them: `[x: number, number]` is `[x: number, y: number]`. A tuple with no labels is left, and a labelled rest element (`...rest: string[]`) counts as labelled.

**Why:** A partly labelled tuple documents some positions and leaves the reader to guess the others.

```codelaw
match:
  kind: tuple_type
  all:
    - has: { any: [{ kind: required_parameter }, { kind: optional_parameter }] }
    - has:
        any:
          - kind: predefined_type
          - kind: type_identifier
          - kind: generic_type
          - kind: array_type
          - kind: literal_type
          - kind: object_type
          - kind: union_type
          - kind: tuple_type
          - kind: optional_type
          - kind: rest_type
          - kind: function_type
          - kind: nested_type_identifier
message: "Label every element of this tuple, or none"
```

## How to fix

1. Give each unlabelled element a label (`name: Type`), or remove the labels the tuple has.

## Examples

### TypeScript

#### Don't

```ts
type Point = [x: number, number];
```

#### Do

```ts
type Point = [x: number, y: number];
```


### TSX

#### Don't

```tsx
type Point = [x: number, number];
```

#### Do

```tsx
type Point = [x: number, y: number];
```
