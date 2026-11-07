---
id: js/public-static-readonly
title: "A public static field is readonly"
languages: [typescript, tsx]
fix: suggest
fix_reason: "code elsewhere may assign the field; the reviewer checks before it becomes readonly"
stability: preview
ported_from: [eslint-plugin-sonarjs/public-static-readonly]
since: 0.5.0
---

A `static` field that is public and has a value is marked `readonly`.

**Why:** A public static field is global state any module can change; `readonly` makes it the constant it usually is.

```codelaw
match:
  kind: public_field_definition
  has: { field: value }
  all:
    - regex: '(^|\s)static\s'
  not:
    any:
      - regex: '\breadonly\b'
      - regex: '^(?:@[^\n]*\s+)*(private|protected)\s'
      - has: { kind: private_property_identifier }
message: "Make this public static field `readonly`"
fix:
  - edit-text: { pattern: '\bstatic(\s+)', replace: 'static readonly${1}' }
```

## Examples

### TypeScript

#### Don't

```ts
class Config {
  public static retries = 3;
}
```

#### Do

```ts
class Config {
  public static readonly retries = 3;
}
```


### TSX

#### Don't

```tsx
class Config {
  public static retries = 3;
}
```

#### Do

```tsx
class Config {
  public static readonly retries = 3;
}
```
