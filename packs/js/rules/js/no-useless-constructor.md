---
id: js/no-useless-constructor
title: "No constructor that only calls super"
languages: [javascript, jsx]
fix: suggest
fix_reason: "an empty constructor in a derived class is not equivalent: removing it starts calling the parent constructor"
stability: preview
ported_from: [eslint/no-useless-constructor]
since: 0.5.0
---

A constructor does not only pass its parameters on to `super`; the TypeScript forms (parameter properties, overloads) are left to a TypeScript rule.

**Why:** The language supplies exactly that constructor, so writing it out adds lines and nothing else.

```codelaw

match:
  kind: method_definition
  has: { field: name, regex: '^constructor$' }
  any:
    - all:
        - has: { field: parameters, regex: '^\(\s*\)$' }
        - has: { field: body, kind: statement_block, has: { kind: expression_statement, pattern: "super();" }, not: { has: { nthChild: 2 } } }
    - all:
        - has: { field: parameters, has: { nthChild: 1, kind: identifier, pattern: $P }, not: { has: { nthChild: 2 } } }
        - has: { field: body, kind: statement_block, has: { kind: expression_statement, pattern: "super($P);" }, not: { has: { nthChild: 2 } } }
  inside: { kind: class_body, inside: { has: { kind: class_heritage } } }
message: "This constructor does what the default one does; remove it"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
class Admin extends User {
  constructor(name) {
    super(name);
  }
  ban() {}
}
```

#### Do

```js
class Admin extends User {
  ban() {}
}
```

### JSX

#### Don't

```jsx
class Admin extends User {
  constructor(name) {
    super(name);
  }
  ban() {}
}
```

#### Do

```jsx
class Admin extends User {
  ban() {}
}
```
