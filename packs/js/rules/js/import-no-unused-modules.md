---
id: js/import-no-unused-modules
title: "No module without exports and no export nothing imports"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: suggest
fix_reason: "an export that is still used in its own file loses its `export`, and one used nowhere is deleted, both as a diff to accept; a module with no exports, or an export the graph cannot edit, offers no fix"
stability: preview
ported_from: [eslint-plugin-import/no-unused-modules, eslint-plugin-import-x/no-unused-modules]
since: 0.5.0
---

Both halves of the upstream rule, on (upstream's options both default to off). **`missingExports`:** a module of the project with no export at all is reported at its first statement; **`unusedExports`:** a named or default export that no other module imports — by name, as `{ default }`, as `ns.x` of a namespace import, through a re-export, or by a literal `import()`/`require()` of the module, which reads every name — is reported at the export. `export * from` passes the names of the module it names on when the re-exporting module is itself read. What is read from outside the project is never reported: an entry point, a package's public surface, a path in `[graph] keep`, a test file and a module that uses dynamic imports (upstream's `src` and `ignoreExports` options are the project graph's entry and keep settings). `graph/no-unused-exports` is the knip-style rule over declarations; this one follows the export names, so it also sees an unused re-export.

**Why:** an export nothing imports is dead code that still looks like API, and a module with no exports is a script that nothing can reuse.

```codelaw
graph:
  module:
    exports: [missing, unused]
message: "`$NAME` is exported but no other module imports it"
fix:
  - when: { local_references: ">0" }
    do: [strip-export-keyword]
  - else:
    do: [remove-declaration]
```

## How to fix

1. Delete the export and the code behind it, or import it where it should be used.
2. A module with no exports is an entry point, or code to delete: list it in `[graph] entry` if it is an entry.

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const used = 1;
export const idle = 2;
// src/main.js
import { used } from "./lib";
export const run = () => used;
```

#### Do

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const used = 1;
// src/main.js
import { used } from "./lib";
export const run = () => used;
```

### JSX

#### Don't

```jsx files
// package.json
{ "name": "app", "private": true, "main": "src/main.jsx" }
// src/lib.jsx
export const used = 1;
export const idle = 2;
// src/main.jsx
import { used } from "./lib";
export const run = () => used;
```

#### Do

```jsx files
// package.json
{ "name": "app", "private": true, "main": "src/main.jsx" }
// src/lib.jsx
export const used = 1;
// src/main.jsx
import { used } from "./lib";
export const run = () => used;
```

### TypeScript

#### Don't

```ts files
// package.json
{ "name": "app", "private": true, "main": "src/main.ts" }
// src/lib.ts
export const used = 1;
export const idle = 2;
// src/main.ts
import { used } from "./lib";
export const run = () => used;
```

#### Do

```ts files
// package.json
{ "name": "app", "private": true, "main": "src/main.ts" }
// src/lib.ts
export const used = 1;
// src/main.ts
import { used } from "./lib";
export const run = () => used;
```

### TSX

#### Don't

```tsx files
// package.json
{ "name": "app", "private": true, "main": "src/main.tsx" }
// src/lib.tsx
export const used = 1;
export const idle = 2;
// src/main.tsx
import { used } from "./lib";
export const run = () => used;
```

#### Do

```tsx files
// package.json
{ "name": "app", "private": true, "main": "src/main.tsx" }
// src/lib.tsx
export const used = 1;
// src/main.tsx
import { used } from "./lib";
export const run = () => used;
```
