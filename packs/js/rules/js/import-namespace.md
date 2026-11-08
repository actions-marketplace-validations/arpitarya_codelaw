---
id: js/import-namespace
title: "A namespace import may only be read at names the module exports"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: suggest
fix_reason: "the export nearest in spelling replaces a plain `ns.x` as a diff to accept; a name near nothing, or one read by a pattern, offers no fix"
stability: preview
ported_from: [eslint-plugin-import/namespace, eslint-plugin-import-x/namespace]
since: 0.5.0
---

For `import * as ns from "./m"`, each `ns.x`, `ns["x"]` and `const { x } = ns` that names something `./m` does not export is reported at the name, and so is a namespace import of a module that exports nothing. A namespace that is passed around whole, or read through a computed name (`ns[name]`), is not judged — upstream's `allowComputed` report, an assignment to a member, and a name two levels deep (`ns.a.b`) are not checked.

**Why:** a member the module does not export is `undefined`; the misspelled read looks like any other until it runs.

```codelaw
graph:
  import:
    namespace: unexported
message: "`$NAME` is not exported by `$SOURCE`"
fix:
  do: [nearest-export]
```

## How to fix

1. Accept the fix where one is offered: `codelaw fix --unsafe` reads the export nearest in spelling.
2. Otherwise read a name the module exports, or add the export.

## Examples

### JavaScript

#### Don't

```js files
// src/util.js
export function formatDate(d) { return d.toISOString(); }
// src/app.js
import * as util from "./util";
console.log(util.formatDat(new Date()));
```

#### Do

```js files
// src/util.js
export function formatDate(d) { return d.toISOString(); }
// src/app.js
import * as util from "./util";
console.log(util.formatDate(new Date()));
```

### JSX

#### Don't

```jsx files
// src/util.jsx
export function formatDate(d) { return d.toISOString(); }
// src/app.jsx
import * as util from "./util";
console.log(util.formatDat(new Date()));
```

#### Do

```jsx files
// src/util.jsx
export function formatDate(d) { return d.toISOString(); }
// src/app.jsx
import * as util from "./util";
console.log(util.formatDate(new Date()));
```

### TypeScript

#### Don't

```ts files
// src/util.ts
export function formatDate(d) { return d.toISOString(); }
// src/app.ts
import * as util from "./util";
console.log(util.formatDat(new Date()));
```

#### Do

```ts files
// src/util.ts
export function formatDate(d) { return d.toISOString(); }
// src/app.ts
import * as util from "./util";
console.log(util.formatDate(new Date()));
```

### TSX

#### Don't

```tsx files
// src/util.tsx
export function formatDate(d) { return d.toISOString(); }
// src/app.tsx
import * as util from "./util";
console.log(util.formatDat(new Date()));
```

#### Do

```tsx files
// src/util.tsx
export function formatDate(d) { return d.toISOString(); }
// src/app.tsx
import * as util from "./util";
console.log(util.formatDate(new Date()));
```
