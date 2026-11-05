---
id: js/no-static-only-class
title: "No class with only static members"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "callers of `MathUtil.double` become callers of `double`"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-static-only-class]
since: 0.5.0
---

A class is not made only of static members.

**Why:** Such a class is never instantiated; module-level functions say the same and tree-shake better.

```codelaw
match:
  kind: class_body
  has:
    kind: method_definition
  not:
    has:
      any:
      - kind: method_definition
        not:
          regex: ^static\b
      - kind: field_definition
        not:
          regex: ^static\b
      - kind: public_field_definition
        not:
          regex: ^static\b
      - kind: class_static_block
  all:
  - not:
      inside:
        has:
          kind: class_heritage
message: A class of static members is a namespace; export plain functions
```

## How to fix

1. Turn each static member into an exported function or constant, and update the callers.

## Examples

### JavaScript

#### Don't

```js
class MathUtil {
  static double(x) {
    return x * 2;
  }
}
```

#### Do

```js
export function double(x) {
  return x * 2;
}
```

### JSX

#### Don't

```jsx
class MathUtil {
  static double(x) {
    return x * 2;
  }
}
```

#### Do

```jsx
export function double(x) {
  return x * 2;
}
```

### TypeScript

#### Don't

```ts
class MathUtil {
  static double(x) {
    return x * 2;
  }
}
```

#### Do

```ts
export function double(x) {
  return x * 2;
}
```

### TSX

#### Don't

```tsx
class MathUtil {
  static double(x) {
    return x * 2;
  }
}
```

#### Do

```tsx
export function double(x) {
  return x * 2;
}
```
