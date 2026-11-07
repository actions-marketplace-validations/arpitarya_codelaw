---
id: js/prefer-type-literal-last
title: "An object type literal after the named members of a union"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-type-literal-last]
since: 0.5.0
---

In a two-member union, an inline object type comes after the named one: `Other | { foo: string }`, not `{ foo: string } | Other`. Member order does not change what a union means. `null` and `undefined` stay last, and a union or intersection beside the literal is left. Upstream also reorders longer unions; this rule handles the two-member form.

**Why:** A multi-line literal in front splits the short names around it; at the end it reads as the detail it is.

```codelaw
match:
  kind: union_type
  not: { inside: { kind: union_type } }
  all:
    - has: { nthChild: 1, kind: object_type, pattern: $LIT }
    - has:
        nthChild: 2
        pattern: $OTHER
        any:
          - kind: type_identifier
          - kind: generic_type
          - kind: predefined_type
            not: { regex: '^(null|undefined)$' }
          - kind: nested_type_identifier
          - kind: literal_type
            not: { regex: '^(null|undefined)$' }
          - kind: array_type
message: "Put the object type literal last"
fix: "$OTHER | $LIT"
```

## Examples

### TypeScript

#### Don't

```ts
type Element = { tag: string } | Text;
```

#### Do

```ts
type Element = Text | { tag: string };
```


### TSX

#### Don't

```tsx
type Element = { tag: string } | Text;
```

#### Do

```tsx
type Element = Text | { tag: string };
```
