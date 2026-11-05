---
id: ts/explicit-member-accessibility
title: "Class members say public, protected or private"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/explicit-member-accessibility]
since: 0.5.0
---

Every class method, field, accessor, constructor and constructor parameter property carries an accessibility modifier. The fix writes `public`, which is what an unmarked member already is. `#private` members, `declare` fields and fields with a decorator are not reported.

**Why:** An explicit modifier makes the class's surface readable at a glance and makes a member's widening a visible change in review.

```codelaw
match:
  any:
    - kind: method_definition
      inside: { kind: class_body }
    - kind: method_signature
      inside: { kind: class_body }
    - kind: abstract_method_signature
      inside: { kind: class_body }
    - kind: public_field_definition
      inside: { kind: class_body }
      not: { any: [{ has: { kind: decorator } }, { regex: '^(?:(?:static|readonly|abstract|override)\s+)*declare\s' }] }
    - kind: required_parameter
      regex: '^(?:readonly|override)\s'
      inside: { kind: formal_parameters, inside: { kind: method_definition, has: { field: name, regex: '^constructor$' } } }
  not:
    any:
      - has: { kind: accessibility_modifier }
      - has: { field: name, kind: private_property_identifier }
message: "Say whether this member is public, protected or private"
fix:
  - insert-before: "public "
```

## Examples

### TypeScript

#### Don't

```ts
class Counter {
  count = 0;
  increment() { this.count += 1; }
}
```

#### Do

```ts
class Counter {
  private count = 0;
  public increment() { this.count += 1; }
}
```

#### Fixed

```ts
class Counter {
  public count = 0;
  public increment() { this.count += 1; }
}
```

### TSX

#### Don't

```tsx
class Counter {
  count = 0;
  increment() { this.count += 1; }
}
```

#### Do

```tsx
class Counter {
  private count = 0;
  public increment() { this.count += 1; }
}
```

#### Fixed

```tsx
class Counter {
  public count = 0;
  public increment() { this.count += 1; }
}
```
