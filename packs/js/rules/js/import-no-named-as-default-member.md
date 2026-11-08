---
id: js/import-no-named-as-default-member
title: "A named export must not be read off the default export"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "the named import is added in the import list and every read of the member changes with it, which is a rewrite of the importing file's uses"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-named-as-default-member, eslint-plugin-import-x/no-named-as-default-member]
since: 0.5.0
---

`x.y` and `const { y } = x`, where `x` is a default import and the module also exports `y` by name, are reported at `y`: `import { y } from` names the export directly. `x.default` is not reported. A module that is not an ES module is left alone.

**Why:** the default object and the named export are two bindings that usually hold one value; reading through the default keeps the whole object alive and hides the dependency from tree-shaking and from this graph.

```codelaw
graph:
  import:
    default_member: named
message: "`$LOCAL` also has a named export `$NAME`; import it as `{ $NAME }` from `$SOURCE`"
```

## How to fix

1. Import the name: `import { find } from "./lib"` and use `find`.
2. If the default is a different value, say so by reading a name the module does not also export.

## Examples

### JavaScript

#### Don't

```js files
// src/lib.js
export const find = (id) => id;
export default { find };
// src/app.js
import lib from "./lib";
console.log(lib.find(1));
```

#### Do

```js files
// src/lib.js
export const find = (id) => id;
export default { find };
// src/app.js
import { find } from "./lib";
console.log(find(1));
```

### JSX

#### Don't

```jsx files
// src/lib.jsx
export const find = (id) => id;
export default { find };
// src/app.jsx
import lib from "./lib";
console.log(lib.find(1));
```

#### Do

```jsx files
// src/lib.jsx
export const find = (id) => id;
export default { find };
// src/app.jsx
import { find } from "./lib";
console.log(find(1));
```

### TypeScript

#### Don't

```ts files
// src/lib.ts
export const find = (id) => id;
export default { find };
// src/app.ts
import lib from "./lib";
console.log(lib.find(1));
```

#### Do

```ts files
// src/lib.ts
export const find = (id) => id;
export default { find };
// src/app.ts
import { find } from "./lib";
console.log(find(1));
```

### TSX

#### Don't

```tsx files
// src/lib.tsx
export const find = (id) => id;
export default { find };
// src/app.tsx
import lib from "./lib";
console.log(lib.find(1));
```

#### Do

```tsx files
// src/lib.tsx
export const find = (id) => id;
export default { find };
// src/app.tsx
import { find } from "./lib";
console.log(find(1));
```
