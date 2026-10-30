---
id: react/prefer-read-only-props
title: "Props types are readonly"
languages: [tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/prefer-read-only-props]
since: 0.5.0
---

Each member of a component's props type is `readonly`: the inline type of a component's first parameter, and any interface or type alias whose name ends in `Props`.

**Why:** Props are read-only at run time; `readonly` makes the compiler say so when code tries to assign one.

```codelaw
match:
  kind: property_signature
  not: { regex: '^readonly\s' }
  any:
    - inside: { kind: interface_body, inside: { kind: interface_declaration, has: { field: name, regex: 'Props$' } } }
    - inside:
        kind: object_type
        any:
          - inside: { kind: type_alias_declaration, has: { field: name, regex: 'Props$' } }
          - inside: { kind: type_annotation, inside: { kind: required_parameter, inside: { kind: formal_parameters, inside: { kind: function_declaration, has: { field: name, regex: '^[A-Z]' } } } } }
message: "Make this prop `readonly`"
fix:
  - insert-before: "readonly "
```

## Examples

### TSX

#### Don't

```tsx
type CardProps = { title: string };
```

#### Do

```tsx
type CardProps = { readonly title: string };
```
