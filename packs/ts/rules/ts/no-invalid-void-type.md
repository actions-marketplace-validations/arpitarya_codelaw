---
id: ts/no-invalid-void-type
title: "void only as a return type"
languages: [typescript, tsx]
fix: manual
fix_reason: "the intended type — `undefined`, or a return type — is the author's"
stability: preview
ported_from: [typescript-eslint/no-invalid-void-type]
since: 0.5.0
---

`void` is used only as a return type or a generic argument, not as a variable, parameter or union member type.

**Why:** Elsewhere `void` is confusing: it means "ignored", not `undefined`.

```codelaw
match:
  kind: predefined_type
  regex: '^void$'
  any:
    - inside: { kind: union_type }
    - inside: { kind: type_annotation, inside: { any: [{kind: variable_declarator}, {kind: required_parameter}, {kind: optional_parameter}, {kind: public_field_definition}, {kind: property_signature}] } }
message: "Use `undefined` here; `void` belongs in a return type"
```

## How to fix

1. Use `undefined` (or remove the type).

## Examples

### TypeScript

#### Don't

```ts
let result: string | void;
```

#### Do

```ts
let result: string | undefined;
```

### TSX

#### Don't

```tsx
let result: string | void;
```

#### Do

```tsx
let result: string | undefined;
```
