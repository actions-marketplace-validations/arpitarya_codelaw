---
id: js/import-named
title: "An imported name must be exported by the module it names"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: suggest
fix_reason: "the export nearest in spelling is offered as a diff to accept (the local name is kept); a name near nothing may be missing from the module, which only its author can say, and offers no fix"
stability: preview
ported_from: [eslint-plugin-import/named, eslint-plugin-import-x/named]
since: 0.5.0
---

A name in `import { x } from "./m"` or `export { x } from "./m"` that the module `./m` does not export — directly, by `export { … }`, or through an `export * from` — is reported at the specifier. Type-only imports are left alone, as is a module that is not an ES module (CommonJS, a script), a TypeScript `export =` module, and one with an `export *` the project does not hold (a package, an unresolved path): none of them lists its names. The `export … from` form is reported at its statement.

**Why:** a name the module does not export is `undefined` at run time in a bundler's loose mode and a link error in a real ES module loader, and it fails only where the import is evaluated.

```codelaw
graph:
  import:
    names: unexported
message: "`$NAME` is not exported by `$SOURCE`"
fix:
  do: [nearest-export]
```

## How to fix

1. Accept the fix where one is offered: `codelaw fix --unsafe` imports the export nearest in spelling and keeps the local name (`import { near as old }`).
2. Otherwise import a name the module exports, or add the export to the module.

## Examples

### JavaScript

#### Don't

```js files
// src/format.js
export function formatDate(d) { return d.toISOString(); }
// src/app.js
import { formatDat as format } from "./format";
console.log(format(new Date()));
```

#### Do

```js files
// src/format.js
export function formatDate(d) { return d.toISOString(); }
// src/app.js
import { formatDate as format } from "./format";
console.log(format(new Date()));
```

### JSX

#### Don't

```jsx files
// src/format.jsx
export function formatDate(d) { return d.toISOString(); }
// src/app.jsx
import { formatDat as format } from "./format";
console.log(format(new Date()));
```

#### Do

```jsx files
// src/format.jsx
export function formatDate(d) { return d.toISOString(); }
// src/app.jsx
import { formatDate as format } from "./format";
console.log(format(new Date()));
```

### TypeScript

#### Don't

```ts files
// src/format.ts
export function formatDate(d) { return d.toISOString(); }
// src/app.ts
import { formatDat as format } from "./format";
console.log(format(new Date()));
```

#### Do

```ts files
// src/format.ts
export function formatDate(d) { return d.toISOString(); }
// src/app.ts
import { formatDate as format } from "./format";
console.log(format(new Date()));
```

### TSX

#### Don't

```tsx files
// src/format.tsx
export function formatDate(d) { return d.toISOString(); }
// src/app.tsx
import { formatDat as format } from "./format";
console.log(format(new Date()));
```

#### Do

```tsx files
// src/format.tsx
export function formatDate(d) { return d.toISOString(); }
// src/app.tsx
import { formatDate as format } from "./format";
console.log(format(new Date()));
```
