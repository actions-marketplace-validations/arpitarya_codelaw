---
id: js/no-accessor-recursion
title: "A getter or setter does not use itself"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the backing field name (`#name`, `_name`) is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-accessor-recursion]
since: 0.5.0
---

A getter or setter does not read or write the property it defines through `this`.

**Why:** It calls itself recursively and throws a stack overflow.

```codelaw
match:
  kind: method_definition
  regex: ^(static\s+)?(get|set)\s
  has:
    field: name
    pattern: $N
  all:
  - has:
      field: body
      has:
        stopBy:
          any:
          - kind: function_expression
          - kind: function_declaration
          - kind: class
        kind: member_expression
        all:
        - has:
            field: object
            kind: this
        - has:
            field: property
            pattern: $N
message: '`this.$N` inside its own accessor calls the accessor again, forever'
```

## How to fix

1. Store the value in a private field (`#name`) and read or write that from the accessor.

## Examples

### JavaScript

#### Don't

```js
class User {
  get name() {
    return this.name;
  }
}
```

#### Do

```js
class User {
  get name() {
    return this._name;
  }
}
```

### JSX

#### Don't

```jsx
class User {
  get name() {
    return this.name;
  }
}
```

#### Do

```jsx
class User {
  get name() {
    return this._name;
  }
}
```

### TypeScript

#### Don't

```ts
class User {
  get name() {
    return this.name;
  }
}
```

#### Do

```ts
class User {
  get name() {
    return this._name;
  }
}
```

### TSX

#### Don't

```tsx
class User {
  get name() {
    return this.name;
  }
}
```

#### Do

```tsx
class User {
  get name() {
    return this._name;
  }
}
```
