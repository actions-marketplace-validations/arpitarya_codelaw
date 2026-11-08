---
id: js/import-no-deprecated
title: "No import of a name marked @deprecated"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "the replacement is named in the `@deprecated` note in prose, and which call replaces which is the author's reading"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import/no-deprecated, eslint-plugin-import-x/no-deprecated]
since: 0.5.0
---

An import of a name whose declaration carries a JSDoc `@deprecated` tag, and each use of the imported binding, is reported with the tag's note (`Deprecated: use fresh`). The tag is read on the declaration the name ends at, through re-exports; `ns.x` of a namespace import is reported at `x`. A tag on the module as a whole is not read. The graph's `graph/no-deprecated-usage` is another rule: it reads codelaw's own deprecations registry.

**Why:** a deprecated name is on its way out, and every import of it is a place that breaks when it goes.

```codelaw
graph:
  import:
    deprecated: jsdoc
message: "`$NAME` is deprecated"
```

## How to fix

1. Use the replacement the `@deprecated` note names.

## Examples

### JavaScript

#### Don't

```js files
// src/lib.js
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.js
import { old } from "./lib";
old();
```

#### Do

```js files
// src/lib.js
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.js
import { fresh } from "./lib";
fresh();
```

### JSX

#### Don't

```jsx files
// src/lib.jsx
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.jsx
import { old } from "./lib";
old();
```

#### Do

```jsx files
// src/lib.jsx
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.jsx
import { fresh } from "./lib";
fresh();
```

### TypeScript

#### Don't

```ts files
// src/lib.ts
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.ts
import { old } from "./lib";
old();
```

#### Do

```ts files
// src/lib.ts
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.ts
import { fresh } from "./lib";
fresh();
```

### TSX

#### Don't

```tsx files
// src/lib.tsx
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.tsx
import { old } from "./lib";
old();
```

#### Do

```tsx files
// src/lib.tsx
/** @deprecated use fresh */
export function old() {}
export function fresh() {}
// src/app.tsx
import { fresh } from "./lib";
fresh();
```
