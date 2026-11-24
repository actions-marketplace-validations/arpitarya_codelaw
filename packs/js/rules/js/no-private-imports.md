---
id: js/no-private-imports
title: "Do not import an export its JSDoc marks `@private` or `@package` from outside its folder"
languages: [javascript, jsx, typescript, tsx]
scope: project
severity: warn
fix: manual
fix_reason: "which public name to use instead, or whether to widen the tag, is the author's to decide"
fix_exception: value
stability: preview
ported_from: [biome/correctness/noPrivateImports]
since: 0.5.2
---

A name whose declaration carries a JSDoc `@private` or `@package` tag is imported only from the folders the tag allows: `@private` from files in the module's own folder, `@package` from its folder and the folders under it. An import from anywhere else is reported, at the name, whether it is a named import, a default import or a namespace member (`ns.name`). The declaration is found through re-exports, so a name an `index` file re-exports from `./impl` is judged by `impl`'s tag. Behaviour of Biome's `noPrivateImports`, read from its documentation and rewritten.

**Why:** the tag says the author may change or remove the name without notice; code outside the folder that imports it breaks on a release the author called compatible.

Not judged: an import written in an `index` file (a folder's `index` is where it chooses what to show), a dynamic `import()`, a `require()`, a type-only import, and an export without a tag (a tag is a whole word: `@packageDocumentation` is not `@package`).

```codelaw
graph:
  import:
    visibility: restricted
message: "`$NAME` is `$TAG` in `$FILE` and is not importable from here"
```

## How to fix

1. Import the public name the module offers for the same thing, usually through its `index`.
2. If there is none, widen the tag (`@package`) or remove it, or copy the few lines you need.

## Examples

### JavaScript

#### Don't

```js files
// src/lib/impl.js
/** @private */
export const secret = 1;
export const open = secret + 1;
// src/app.js
import { secret } from "./lib/impl";
console.log(secret);
```

#### Do

```js files
// src/lib/impl.js
/** @private */
export const secret = 1;
export const open = secret + 1;
// src/app.js
import { open } from "./lib/impl";
console.log(open);
```

### JSX

#### Don't

```jsx files
// src/lib/impl.jsx
/** @private */
export const secret = 1;
export const open = secret + 1;
// src/app.jsx
import { secret } from "./lib/impl";
console.log(secret);
```

#### Do

```jsx files
// src/lib/impl.jsx
/** @private */
export const secret = 1;
export const open = secret + 1;
// src/app.jsx
import { open } from "./lib/impl";
console.log(open);
```

### TypeScript

#### Don't

```ts files
// src/lib/impl.ts
/** @private */
export const secret: number = 1;
export const open = secret + 1;
// src/app.ts
import { secret } from "./lib/impl";
console.log(secret);
```

#### Do

```ts files
// src/lib/impl.ts
/** @private */
export const secret: number = 1;
export const open = secret + 1;
// src/app.ts
import { open } from "./lib/impl";
console.log(open);
```

### TSX

#### Don't

```tsx files
// src/lib/impl.tsx
/** @private */
export const secret: number = 1;
export const open = secret + 1;
// src/app.tsx
import { secret } from "./lib/impl";
console.log(secret);
```

#### Do

```tsx files
// src/lib/impl.tsx
/** @private */
export const secret: number = 1;
export const open = secret + 1;
// src/app.tsx
import { open } from "./lib/impl";
console.log(open);
```

## Differences from the source

Biome's `noPrivateImports` takes the tag's folders from its own project layout. This rule reads the tag and the two folder rules above; `@public` and an untagged export are importable from anywhere.
