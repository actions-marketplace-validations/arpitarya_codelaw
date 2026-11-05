---
id: js/no-redeclare
title: "No variable is declared twice"
languages: [javascript, jsx]
fix: auto
fix_reason: "a `var` redeclaration of a name already declared in the scope is just an assignment, so `var x = e;` becomes `x = e;` and a bare `var x;` is dropped; other redeclarations (let, function, destructured, multi-declarator) carry no edit"
stability: preview
ported_from: [eslint/no-redeclare]
since: 0.1.0
---

A name is declared once per scope: a second `var` or `function` with the same name is not allowed.

**Why:** Redeclaring silently merges two declarations, so one of them is usually a mistake.

```codelaw
match:
  kind: identifier
  any:
    - inside:
        kind: variable_declarator
        stopBy: neighbor
        pattern: $D1
        has: { field: value, pattern: $V1 }
        inside:
          kind: variable_declaration
          stopBy: neighbor
          pattern: $DECL1
          not: { has: { kind: variable_declarator, nthChild: 2 } }
    - inside:
        kind: variable_declarator
        stopBy: neighbor
        not: { has: { field: value, pattern: $V2 } }
        inside:
          kind: variable_declaration
          stopBy: neighbor
          pattern: $DECL2
          not: { has: { kind: variable_declarator, nthChild: 2 } }
    - kind: identifier
where:
  $$MATCH: [declaration, redeclared]
message: "`$$MATCH` is already declared in this scope"
fix:
  - replace: { target: $DECL1, with: "$D1;" }
  - remove: $DECL2
```

## How to fix

1. If it's the same variable, drop the second `var` and assign.
2. If it's a different one, rename it.

## Examples

### JavaScript

#### Don't

```js
var count = 1;
var count = 2;
```

#### Do

```js
var count = 1;
count = 2;
```

### JSX

#### Don't

```jsx
var count = 1;
var count = 2;
```

#### Do

```jsx
var count = 1;
count = 2;
```
