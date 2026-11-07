---
id: js/redundant-type-aliases
title: "No type alias that only renames a built-in type"
languages: [typescript, tsx]
fix: manual
fix_reason: "the alias's uses are replaced by hand, file by file"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/redundant-type-aliases]
since: 0.5.0
---

A type alias does not only rename a built-in type: `type Name = string` adds a name, not a type.

**Why:** The alias makes a reader look it up to find a plain `string`, and checks nothing a `string` wouldn't.

```codelaw
match:
  kind: type_alias_declaration
  has: { field: value, kind: predefined_type }
  not: { has: { field: type_parameters, kind: type_parameters } }
message: "This alias only renames a built-in type"
```

## How to fix

1. Use the built-in type where the alias is used, and delete the alias — or make the alias a real type (a branded type, a union).

## Examples

### TypeScript

#### Don't

```ts
type UserId = string;
```

#### Do

```ts
type UserId = string & { readonly brand: "UserId" };
```


### TSX

#### Don't

```tsx
type UserId = string;
```

#### Do

```tsx
type UserId = string & { readonly brand: "UserId" };
```
