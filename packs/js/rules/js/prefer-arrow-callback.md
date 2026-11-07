---
id: js/prefer-arrow-callback
title: "Arrow functions for callbacks"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an arrow function cannot be called with `new` and has no `prototype`, which a callee could rely on; a function with type parameters is rewritten by hand"
stability: preview
ported_from: [eslint/prefer-arrow-callback]
since: 0.5.0
---

An anonymous function expression passed as an argument, that does not use `this`, `arguments`, `super` or `new.target`, is an arrow function.

**Why:** An arrow function is shorter and cannot be called with a surprising `this`.

```codelaw
match:
  kind: function_expression
  not:
    has:
      field: name
      regex: .
  inside:
    kind: arguments
  all:
  - not:
      regex: ^(async\s+)?function\s*\*
  - not:
      has:
        stopBy:
          any:
          - kind: function_expression
          - kind: function_declaration
          - kind: method_definition
          - kind: class
        any:
        - kind: this
        - kind: super
        - kind: identifier
          regex: ^arguments$
        - kind: meta_property
  - not: { has: { kind: type_parameters } }
  - any:
      - all: [{ not: { regex: ^async } }, { has: { kind: type_annotation, pattern: $R } }, { has: { field: parameters, pattern: $P } }, { has: { field: body, pattern: $B } }, { pattern: $TYPED }]
      - all: [{ not: { regex: ^async } }, { not: { has: { kind: type_annotation } } }, { has: { field: parameters, pattern: $P } }, { has: { field: body, pattern: $B } }, { pattern: $PLAIN }]
      - all: [{ regex: ^async }, { not: { has: { kind: type_annotation } } }, { has: { field: parameters, pattern: $P } }, { has: { field: body, pattern: $B } }, { pattern: $ASYNC }]
      - regex: ''
message: Use an arrow function for this callback
fix:
  - replace: { target: $TYPED, with: "$P$R => $B" }
  - replace: { target: $PLAIN, with: "$P => $B" }
  - replace: { target: $ASYNC, with: "async $P => $B" }
```

## How to fix

1. Rewrite `function (a, b) { … }` as `(a, b) => { … }`.

## Examples

### JavaScript

#### Don't

```js
items.map(function (x) {
  return x * 2;
});
```

#### Do

```js
items.map((x) => {
  return x * 2;
});
```

### JSX

#### Don't

```jsx
items.map(function (x) {
  return x * 2;
});
```

#### Do

```jsx
items.map((x) => {
  return x * 2;
});
```

### TypeScript

#### Don't

```ts
items.map(function (x) {
  return x * 2;
});
```

#### Do

```ts
items.map((x) => {
  return x * 2;
});
```

### TSX

#### Don't

```tsx
items.map(function (x) {
  return x * 2;
});
```

#### Do

```tsx
items.map((x) => {
  return x * 2;
});
```
