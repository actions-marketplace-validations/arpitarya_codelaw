---
id: js/max-union-size
title: "No union of more than three types outside an alias"
languages: [typescript, tsx]
fix: manual
fix_reason: "the alias's name is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/max-union-size]
since: 0.5.0
---

A union written in place has at most three members (SonarJS's default); a longer one is given a name with a type alias. Unions that are an alias's own value are left alone.

**Why:** A long union repeated at each use is hard to read and easy to let drift.

```codelaw
match:
  kind: union_type
  has: { kind: union_type, has: { kind: union_type } }
  not:
    any:
      - inside: { kind: union_type }
      - inside: { kind: type_alias_declaration, field: value }
message: "Give this union a name with a type alias"
```

## How to fix

1. Declare `type <Name> = …` with the union and use the name here.

## Examples

### TypeScript

#### Don't

```ts
function pick(mode: "a" | "b" | "c" | "d") {}
```

#### Do

```ts
type Mode = "a" | "b" | "c" | "d";
function pick(mode: Mode) {}
```


### TSX

#### Don't

```tsx
function pick(mode: "a" | "b" | "c" | "d") {}
```

#### Do

```tsx
type Mode = "a" | "b" | "c" | "d";
function pick(mode: Mode) {}
```
