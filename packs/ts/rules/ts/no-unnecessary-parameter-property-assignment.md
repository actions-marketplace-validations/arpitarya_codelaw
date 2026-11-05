---
id: ts/no-unnecessary-parameter-property-assignment
title: "No assignment a parameter property already makes"
languages: [typescript, tsx]
fix: auto
fix_reason: "auto when the assignment is the constructor's first statement, so nothing can change the value before it; a later one is reported as a suggestion"
stability: preview
ported_from: [typescript-eslint/no-unnecessary-parameter-property-assignment]
since: 0.5.0
---

A constructor does not assign `this.x = x` for a parameter property `x`.

**Why:** The parameter property already assigns it.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment_expression, pattern: this.$X = $X}
  inside:
    kind: statement_block
    inside:
      kind: method_definition
      all:
      - has: {field: name, regex: ^constructor$}
      - has:
          field: parameters
          has:
            kind: required_parameter
            all:
            - has: {kind: accessibility_modifier}
            - has: {field: pattern, pattern: $X}
message: '`this.$X = $X` repeats what the parameter property does'
auto_when: { nthChild: 1 }
fix:
  - remove: $$MATCH
```

## How to fix

1. Remove the assignment.

## Examples

### TypeScript

#### Don't

```ts
class A {
  constructor(private x: number) {
    this.x = x;
  }
}
```

#### Fixed

```ts
class A {
  constructor(private x: number) {
  }
}
```

#### Do

```ts
class A {
  constructor(private x: number) {}
}
```

### TSX

#### Don't

```tsx
class A {
  constructor(private x: number) {
    this.x = x;
  }
}
```

#### Fixed

```tsx
class A {
  constructor(private x: number) {
  }
}
```

#### Do

```tsx
class A {
  constructor(private x: number) {}
}
```
