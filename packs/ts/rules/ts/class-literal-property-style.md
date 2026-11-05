---
id: ts/class-literal-property-style
title: "A literal getter is a readonly field"
languages: [typescript, tsx]
fix: suggest
fix_reason: "a field is set on each instance and is visible to `Object.keys`, where the getter lived on the prototype"
stability: preview
ported_from: [typescript-eslint/class-literal-property-style]
since: 0.5.0
---

A getter whose whole body returns a literal (`get kind() { return "user"; }`) is written as a `readonly` field instead. A getter with a setter of the same name is left alone.

**Why:** A getter that always returns the same literal is a constant; a `readonly` field says so in one line and keeps the literal type.

```codelaw
match:
  kind: method_definition
  regex: '^get\s'
  has: { field: name, pattern: $N }
  all:
    - has:
        field: body
        not: { has: { nthChild: 2 } }
        has:
          kind: return_statement
          has:
            pattern: $V
            any:
              - kind: string
              - kind: number
              - kind: "true"
              - kind: "false"
              - kind: "null"
              - { kind: template_string, not: { has: { kind: template_substitution } } }
    - not:
        inside:
          kind: class_body
          has:
            kind: method_definition
            regex: '^(?:(?:public|protected|private|static|override)\s+)*set\s'
            has: { field: name, pattern: $N }
message: "`$N` returns a literal; make it a `readonly` field"
fix: "readonly $N = $V;"
```

## Examples

### TypeScript

#### Don't

```ts
class User {
  get kind() { return "user"; }
}
```

#### Do

```ts
class User {
  readonly kind = "user";
}
```

### TSX

#### Don't

```tsx
class User {
  get kind() { return "user"; }
}
```

#### Do

```tsx
class User {
  readonly kind = "user";
}
```
