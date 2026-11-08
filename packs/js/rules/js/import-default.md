---
id: js/import-default
title: "A default import needs a default export"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "either the module gets a default export or the import becomes a named one, and which is meant is the author's call across two files"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/default, eslint-plugin-import-x/default]
since: 0.5.0
---

`import x from "./m"` where the module `./m` has no `export default` is reported at the default specifier. `export * from` does not pass a default export on, so a barrel that only re-exports with `*` has none. A module that is not an ES module (CommonJS, a script), a TypeScript `export =` module, and one with an `export *` the project does not hold are left alone.

**Why:** a default import of a module without one is `undefined` (or a link error), and every use of the binding fails far from the import.

```codelaw
graph:
  import:
    default: missing
message: "`$SOURCE` has no default export"
```

## How to fix

1. Import the name the module exports: `import { helper } from "./helper"`.
2. Or give the module the default export the import expects.

## Examples

### JavaScript

#### Don't

```js files
// src/helper.js
export const helper = () => 1;
// src/app.js
import helper from "./helper";
console.log(helper());
```

#### Do

```js files
// src/helper.js
export const helper = () => 1;
// src/app.js
import { helper } from "./helper";
console.log(helper());
```

### JSX

#### Don't

```jsx files
// src/helper.jsx
export const helper = () => 1;
// src/app.jsx
import helper from "./helper";
console.log(helper());
```

#### Do

```jsx files
// src/helper.jsx
export const helper = () => 1;
// src/app.jsx
import { helper } from "./helper";
console.log(helper());
```

### TypeScript

#### Don't

```ts files
// src/helper.ts
export const helper = () => 1;
// src/app.ts
import helper from "./helper";
console.log(helper());
```

#### Do

```ts files
// src/helper.ts
export const helper = () => 1;
// src/app.ts
import { helper } from "./helper";
console.log(helper());
```

### TSX

#### Don't

```tsx files
// src/helper.tsx
export const helper = () => 1;
// src/app.tsx
import helper from "./helper";
console.log(helper());
```

#### Do

```tsx files
// src/helper.tsx
export const helper = () => 1;
// src/app.tsx
import { helper } from "./helper";
console.log(helper());
```
