---
id: js/no-constructor-return
title: "A constructor returns no value"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "`return this;` and a primitive return in a class with no `extends` are ignored by the runtime, so dropping the value changes nothing; a primitive in a derived class is a suggestion and an object return has no edit"
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
  any:
    - all: [{ pattern: $THIS }, { pattern: "return this;" }]
    - all:
        - { pattern: $PRIM }
        - has: { any: [{ kind: number }, { kind: string }, { kind: "true" }, { kind: "false" }, { kind: "null" }] }
    - { kind: return_statement }
auto_when:
  any:
    - pattern: "return this;"
    - all:
        - has: { any: [{ kind: number }, { kind: string }, { kind: "true" }, { kind: "false" }, { kind: "null" }] }
        - not: { inside: { stopBy: end, any: [{kind: class_declaration}, {kind: class}], has: { kind: class_heritage } } }
message: "Do not return a value from a constructor"
fix:
  - replace: { target: $THIS, with: "return;" }
  - replace: { target: $PRIM, with: "return;" }
```

## How to fix

1. Remove the value, or move the logic into a static factory method.

## Examples

### JavaScript

#### Don't

```js
class A {
  constructor() {
    return this;
  }
}
```

#### Fixed

```js
class A {
  constructor() {
    return;
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
    return this;
  }
}
```

#### Fixed

```jsx
class A {
  constructor() {
    return;
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
    return this;
  }
}
```

#### Fixed

```ts
class A {
  constructor() {
    return;
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
    return this;
  }
}
```

#### Fixed

```tsx
class A {
  constructor() {
    return;
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
