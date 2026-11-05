---
id: ts/parameter-properties
title: "No parameter properties"
languages: [typescript, tsx]
fix: manual
fix_reason: "moving the property into the class body adds a field and an assignment"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [typescript-eslint/parameter-properties]
since: 0.5.0
---

Class properties are declared in the class body, not as constructor parameter properties (`constructor(private x: T)`).

**Why:** A parameter property hides a field declaration in the constructor's signature, and is not plain JavaScript.

```codelaw
match:
  kind: required_parameter
  has: { kind: accessibility_modifier }
  inside: { kind: formal_parameters, inside: { kind: method_definition, has: { field: name, regex: '^constructor$' } } }
message: "Declare this property in the class body instead of the constructor's parameters"
```

## How to fix

1. Declare the field in the class and assign it in the constructor.

## Examples

### TypeScript

#### Don't

```ts
class Repo {
  constructor(private db: Db) {}
}
```

#### Do

```ts
class Repo {
  private db: Db;
  constructor(db: Db) {
    this.db = db;
  }
}
```

### TSX

#### Don't

```tsx
class Repo {
  constructor(private db: Db) {}
}
```

#### Do

```tsx
class Repo {
  private db: Db;
  constructor(db: Db) {
    this.db = db;
  }
}
```
