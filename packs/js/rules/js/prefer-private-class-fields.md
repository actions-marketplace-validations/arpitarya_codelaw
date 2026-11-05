---
id: js/prefer-private-class-fields
title: "#private rather than _private"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "every use of the member must change, and outside code that reads it breaks"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-private-class-fields]
since: 0.5.0
---

A class field or method is not named with a leading underscore to mark it private.

**Why:** `#name` is enforced by the language; `_name` is only a convention anyone can break.

```codelaw

match:
  any:
    - { kind: field_definition, has: { field: property, regex: '^_[A-Za-z]' } }
    - { kind: public_field_definition, has: { field: name, regex: '^_[A-Za-z]' }, not: { has: { kind: accessibility_modifier } } }
    - { kind: method_definition, has: { field: name, regex: '^_[A-Za-z]' } }
message: "Make it private with `#`, not a leading underscore"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
class Counter {
  _count = 0;
}
```

#### Do

```js
class Counter {
  #count = 0;
}
```

### JSX

#### Don't

```jsx
class Counter {
  _count = 0;
}
```

#### Do

```jsx
class Counter {
  #count = 0;
}
```

### TypeScript

#### Don't

```ts
class Counter {
  _count = 0;
}
```

#### Do

```ts
class Counter {
  #count = 0;
}
```

### TSX

#### Don't

```tsx
class Counter {
  _count = 0;
}
```

#### Do

```tsx
class Counter {
  #count = 0;
}
```
