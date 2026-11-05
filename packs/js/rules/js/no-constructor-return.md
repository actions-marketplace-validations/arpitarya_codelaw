---
id: js/no-constructor-return
title: "A constructor returns no value"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the returned value may belong in a factory function"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/no-constructor-return]
since: 0.5.0
---

A class constructor does not `return` a value.

**Why:** Returning an object from a constructor replaces the instance `new` made, which surprises every caller.

```codelaw
match:
  kind: return_statement
  regex: '^return\s*[^\s;]'
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: method_definition}] }
    kind: method_definition
    has: { field: name, regex: '^constructor$' }
message: "Do not return a value from a constructor"
```

## How to fix

1. Remove the value, or move the logic into a static factory method.

## Examples

### JavaScript

#### Don't

```js
class A {
  constructor() {
    return {};
  }
}
```

#### Do

```js
class A {
  constructor() {
    this.x = 1;
  }
}
```

### JSX

#### Don't

```jsx
class A {
  constructor() {
    return {};
  }
}
```

#### Do

```jsx
class A {
  constructor() {
    this.x = 1;
  }
}
```

### TypeScript

#### Don't

```ts
class A {
  constructor() {
    return {};
  }
}
```

#### Do

```ts
class A {
  constructor() {
    this.x = 1;
  }
}
```

### TSX

#### Don't

```tsx
class A {
  constructor() {
    return {};
  }
}
```

#### Do

```tsx
class A {
  constructor() {
    this.x = 1;
  }
}
```
