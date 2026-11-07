---
id: js/class-reference-in-static-methods
title: "this and super in a static method, not the class names"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`this` follows the class the method is called on, a subclass included, where the name always means the base class; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/class-reference-in-static-methods]
since: 0.5.0
---

Inside a static method, a member of the class itself is read through `this` (`this.create()`, not `Foo.create()`), and a member of the class it extends through `super` (`super.create()`, not `Bar.create()`). Functions nested in the method have their own `this` and are left.

**Why:** One spelling throughout says plainly that the static members are meant to follow subclass dispatch.

```codelaw
utils:
  nested_fn:
    any:
      - kind: function_expression
      - kind: function_declaration
      - kind: class_declaration
      - kind: class
  static_method:
    kind: method_definition
    regex: '^static\s'
match:
  kind: member_expression
  any:
    - all:
        - has: { field: object, kind: identifier, pattern: $C }
        - inside:
            stopBy: end
            kind: class_declaration
            has: { field: name, pattern: $C }
        - has: { field: object, pattern: $O1 }
    - all:
        - has: { field: object, kind: identifier, pattern: $S }
        - inside:
            stopBy: end
            kind: class_declaration
            has: { kind: class_heritage, has: { stopBy: end, kind: identifier, pattern: $S } }
        - has: { field: object, pattern: $O2 }
  inside:
    stopBy: { any: [{ matches: static_method }, { matches: nested_fn }] }
    matches: static_method
  not:
    any:
      - inside: { kind: assignment_expression, field: left }
      - inside: { kind: augmented_assignment_expression }
      - inside: { kind: update_expression }
message: "Read the class through `this` or `super` in a static method"
fix:
  - replace: { target: $O1, with: this }
  - replace: { target: $O2, with: super }
```

## Examples

### JavaScript

#### Don't

```js
class Store extends Base {
  static open() {
    return Store.create(Base.defaults());
  }
}
```

#### Do

```js
class Store extends Base {
  static open() {
    return this.create(super.defaults());
  }
}
```


### JSX

#### Don't

```jsx
class Store extends Base {
  static open() {
    return Store.create(Base.defaults());
  }
}
```

#### Do

```jsx
class Store extends Base {
  static open() {
    return this.create(super.defaults());
  }
}
```


### TypeScript

#### Don't

```ts
class Store extends Base {
  static open() {
    return Store.create(Base.defaults());
  }
}
```

#### Do

```ts
class Store extends Base {
  static open() {
    return this.create(super.defaults());
  }
}
```


### TSX

#### Don't

```tsx
class Store extends Base {
  static open() {
    return Store.create(Base.defaults());
  }
}
```

#### Do

```tsx
class Store extends Base {
  static open() {
    return this.create(super.defaults());
  }
}
```
