---
id: node/no-missing-import
title: "Imported project paths must exist"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: suggest
fix_reason: "the specifier is pointed at the project file nearest in spelling that exists, or one of the nearest when several tie (choices); a diff to accept, as the module may have moved elsewhere or need restoring. No file near in spelling offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint-plugin-n/no-missing-import]
since: 0.1.0
---

An `import`, `export … from` or `import()` path of the project — relative, absolute or a tsconfig `paths` alias — names a file that exists.

**Why:** a path that names no file fails at build or load time, and only on the code path that reaches it.

```codelaw
graph:
  import:
    form: [import, export, dynamic]
    resolves: false
message: "`$SOURCE` names no file of this project"
fix:
  do: [nearest-path]
```

## How to fix

1. Accept the fix where it is offered: `codelaw fix --unsafe` points the specifier at the nearest file that exists (with choices, pick one).
2. Otherwise point it at the file that exists (check the extension and the folder), or restore the missing file.
3. A path to a generated file the repo doesn't hold is reported too: generate it before `codelaw check`, or suppress the line with a reason.

## Examples

### JavaScript

#### Don't

```js files
// src/app.js
import { helper } from "./helpers";
console.log(helper);
// src/helper.js
export const helper = 1;
```

#### Do

```js files
// src/app.js
import { helper } from "./helper";
console.log(helper);
// src/helper.js
export const helper = 1;
```

### JSX

#### Don't

```jsx files
// src/app.jsx
import { helper } from "./helpers";
export const App = () => <p>{String(helper)}</p>;
// src/helper.jsx
export const helper = 1;
```

#### Do

```jsx files
// src/app.jsx
import { helper } from "./helper";
export const App = () => <p>{String(helper)}</p>;
// src/helper.jsx
export const helper = 1;
```

### TypeScript

#### Don't

```ts files
// src/app.ts
import { helper } from "./helpers";
console.log(helper);
// src/helper.ts
export const helper = 1;
```

#### Do

```ts files
// src/app.ts
import { helper } from "./helper";
console.log(helper);
// src/helper.ts
export const helper = 1;
```

### TSX

#### Don't

```tsx files
// src/app.tsx
import { helper } from "./helpers";
export const App = () => <p>{String(helper)}</p>;
// src/helper.tsx
export const helper = 1;
```

#### Do

```tsx files
// src/app.tsx
import { helper } from "./helper";
export const App = () => <p>{String(helper)}</p>;
// src/helper.tsx
export const helper = 1;
```
