---
id: js/import-max-dependencies
title: "At most ten imports in a module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the module is a design decision"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/max-dependencies]
since: 0.5.0
---

A module has at most ten `import` statements; the eleventh is reported. The limit is upstream's default and is fixed; upstream also counts `require()` calls and counts each module once, this rule counts `import` statements.

**Why:** A module that depends on many others usually does several jobs: it is hard to test, and a change anywhere it imports from can break it.

```codelaw
match:
  kind: import_statement
  nthChild:
    position: 11
    ofRule: { kind: import_statement }
message: "This module has more than ten imports; split it into smaller modules"
```

## How to fix

1. Group the imports by the job each serves in this module.
2. Move each job into its own module with the imports it needs, and import the new modules here.

## Examples

### JavaScript

#### Don't

```js
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
import k from "k";
use(a, b, c, d, e, f, g, h, i, j, k);
```

#### Do

```js
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
use(a, b, c, d, e, f, g, h, i, j);
```

### JSX

#### Don't

```jsx
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
import k from "k";
use(a, b, c, d, e, f, g, h, i, j, k);
```

#### Do

```jsx
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
use(a, b, c, d, e, f, g, h, i, j);
```

### TypeScript

#### Don't

```ts
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
import k from "k";
use(a, b, c, d, e, f, g, h, i, j, k);
```

#### Do

```ts
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
use(a, b, c, d, e, f, g, h, i, j);
```

### TSX

#### Don't

```tsx
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
import k from "k";
use(a, b, c, d, e, f, g, h, i, j, k);
```

#### Do

```tsx
import a from "a";
import b from "b";
import c from "c";
import d from "d";
import e from "e";
import f from "f";
import g from "g";
import h from "h";
import i from "i";
import j from "j";
use(a, b, c, d, e, f, g, h, i, j);
```
