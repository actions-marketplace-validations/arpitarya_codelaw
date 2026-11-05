---
id: ts/no-unnecessary-parameter-property-assignment
title: "No assignment a parameter property already makes"
languages: [typescript, tsx]
fix: manual
fix_reason: "removing the assignment is safe only if nothing changes the value in between"
fix_pending: "auto-subset"
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

#### Do

```tsx
class A {
  constructor(private x: number) {}
}
```
