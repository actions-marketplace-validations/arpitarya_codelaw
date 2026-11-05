---
id: node/no-mixed-requires
title: "Don't mix requires with other declarations"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the declaration needs its keyword repeated per declarator, which no declarative edit can write"
fix_pending: "suggest; needs computed-edit (W-162)"
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
message: "Put the `require()` in a declaration of its own"
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
