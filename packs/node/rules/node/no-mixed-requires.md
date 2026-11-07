---
id: node/no-mixed-requires
title: "Don't mix requires with other declarations"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the two declarators become two statements in their order, which a comment between them or a type on the declaration does not survive unchanged; a declaration of more than two, an exported one or one in a `for` is split by hand"
stability: preview
ported_from: [eslint-plugin-n/no-mixed-requires]
since: 0.5.0
---

One `const`/`let`/`var` statement either requires modules or declares other values, not both (the upstream default: no grouping or call checks).

**Why:** A `require()` hidden among other declarators is easy to miss when reading a file's dependencies.

```codelaw
utils:
  req:
    any:
      - pattern: require($$$A)
      - pattern: require($$$A).$P
match:
  any: [{kind: lexical_declaration}, {kind: variable_declaration}]
  all:
    - has: { kind: variable_declarator, has: { field: value, matches: req } }
    - has: { kind: variable_declarator, has: { field: value, not: { matches: req } } }
    - any:
        - all:
            - pattern: $DECL
            - regex: '^(const|let|var)\s'
            - any: [{ inside: { kind: program } }, { inside: { kind: statement_block } }]
            - has: { kind: variable_declarator, nthChild: 1, pattern: $D1 }
            - has: { kind: variable_declarator, nthChild: 2, pattern: $D2 }
            - not: { has: { kind: variable_declarator, nthChild: 3 } }
            - not: { has: { kind: comment } }
        - regex: ''
transform:
  KW: { replace: { source: $DECL, replace: '^(const|let|var)[\s\S]*$', by: '$1' } }
message: "Put the `require()` in a declaration of its own"
fix:
  - replace: { target: $DECL, with: "$KW $D1;" }
  - insert-after: { target: $DECL, text: "$KW $D2;", line: true }
```

## How to fix

1. Move each `require()` declarator into its own `const` statement, and the other values into theirs.

## Examples

### JavaScript

#### Don't

```js
const fs = require("fs"),
  retries = 3;
```

#### Do

```js
const fs = require("fs");
const retries = 3;
```

### JSX

#### Don't

```jsx
const fs = require("fs"),
  retries = 3;
```

#### Do

```jsx
const fs = require("fs");
const retries = 3;
```

### TypeScript

#### Don't

```ts
const fs = require("fs"),
  retries = 3;
```

#### Do

```ts
const fs = require("fs");
const retries = 3;
```

### TSX

#### Don't

```tsx
const fs = require("fs"),
  retries = 3;
```

#### Do

```tsx
const fs = require("fs");
const retries = 3;
```
