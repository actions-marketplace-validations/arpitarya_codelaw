---
id: js/import-no-cycle
title: "A module must not import its way back to itself"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "breaking a cycle moves code into a third module or inverts a dependency, which is a change of structure across files"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-cycle, eslint-plugin-import-x/no-cycle]
since: 0.5.0
---

Each `import`, `export … from` and `import()` of a module of the project whose target imports the importing module again — through any number of modules — is reported at the specifier, with the shortest way back (`Dependency cycle via b.js -> a.js`). `import type` and an import whose every specifier is a type are erased at build time and close no cycle. `require()` calls are not followed, and upstream's `maxDepth`, `ignoreExternal` and `allowUnsafeDynamicCyclicDependency` options are not built.

**Why:** a module in a cycle sees its partner's exports half-initialised, so a name that works in one import order is `undefined` in another.

```codelaw
graph:
  cycle:
    form: [import, export, dynamic]
message: "This import leads back to this module"
```

## How to fix

1. Move what both modules need into a third module that imports neither.
2. Or pass the dependency in as an argument instead of importing it.

## Examples

### JavaScript

#### Don't

```js files
// src/a.js
import { b } from "./b";
export const a = () => b;
// src/b.js
import { a } from "./a";
export const b = () => a;
```

#### Do

```js files
// src/shared.js
export const shared = 1;
// src/a.js
import { shared } from "./shared";
export const a = () => shared;
// src/b.js
import { a } from "./a";
export const b = () => a;
```

### JSX

#### Don't

```jsx files
// src/a.jsx
import { b } from "./b";
export const a = () => b;
// src/b.jsx
import { a } from "./a";
export const b = () => a;
```

#### Do

```jsx files
// src/shared.jsx
export const shared = 1;
// src/a.jsx
import { shared } from "./shared";
export const a = () => shared;
// src/b.jsx
import { a } from "./a";
export const b = () => a;
```

### TypeScript

#### Don't

```ts files
// src/a.ts
import { b } from "./b";
export const a = () => b;
// src/b.ts
import { a } from "./a";
export const b = () => a;
```

#### Do

```ts files
// src/shared.ts
export const shared = 1;
// src/a.ts
import { shared } from "./shared";
export const a = () => shared;
// src/b.ts
import { a } from "./a";
export const b = () => a;
```

### TSX

#### Don't

```tsx files
// src/a.tsx
import { b } from "./b";
export const a = () => b;
// src/b.tsx
import { a } from "./a";
export const b = () => a;
```

#### Do

```tsx files
// src/shared.tsx
export const shared = 1;
// src/a.tsx
import { shared } from "./shared";
export const a = () => shared;
// src/b.tsx
import { a } from "./a";
export const b = () => a;
```
