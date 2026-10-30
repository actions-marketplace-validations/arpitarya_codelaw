---
id: js/import-no-amd
title: "No AMD define or require"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning an AMD module into an ES module rewrites its whole body and every loader config that names it"
stability: preview
ported_from: [eslint-plugin-import/no-amd]
since: 0.5.0
---

Modules use `import` and `export`, not AMD's `define([deps], factory)` or `require([deps], callback)` at the top level of the file.

**Why:** AMD needs a loader such as RequireJS at run time, and bundlers and type checkers cannot see its dependencies statically.

```codelaw
match:
  kind: call_expression
  has: { field: function, regex: ^(define|require)$, pattern: $F }
  inside:
    kind: expression_statement
    inside: { kind: program }
  all:
    - has:
        field: arguments
        all:
          - has: { kind: array, nthChild: 1 }
          - has: { nthChild: 2 }
          - not: { has: { nthChild: 3 } }
message: "`$F([...], …)` is AMD; use `import` and `export`"
```

## How to fix

1. Turn each dependency in the array into an `import` of the same path, named as the factory's parameter.
2. Move the factory's body to the top level, and turn its `return` into `export default`.

## Examples

### JavaScript

#### Don't

```js
define(["./dep"], function (dep) {
  return dep.make();
});
```

#### Do

```js
import dep from "./dep";
export default dep.make();
```

### JSX

#### Don't

```jsx
define(["./dep"], function (dep) {
  return dep.make();
});
```

#### Do

```jsx
import dep from "./dep";
export default dep.make();
```

### TypeScript

#### Don't

```ts
define(["./dep"], function (dep) {
  return dep.make();
});
```

#### Do

```ts
import dep from "./dep";
export default dep.make();
```

### TSX

#### Don't

```tsx
define(["./dep"], function (dep) {
  return dep.make();
});
```

#### Do

```tsx
import dep from "./dep";
export default dep.make();
```
