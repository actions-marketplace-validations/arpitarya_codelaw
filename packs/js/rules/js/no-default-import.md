---
id: js/no-default-import
title: "Import by name, not the default"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "where the imported project module's default export is a binding it also exports by name, the import names it: `import { name as x }`; a diff to accept. A package, or a module whose default has no named twin, offers no fix (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [tslint/no-default-import]
since: 0.5.0
---

An `import` names the binding it takes (`import { parse } from ...`), not the module's default export. The import-side twin of `js/no-default-export`.

**Why:** A default import can be given any local name, so the same thing is called different names across the codebase, and renaming the export doesn't reach its importers.

```codelaw
native: js/no-default-import
```

## How to fix

1. Accept the fix where it is offered: `codelaw fix --unsafe` imports the module's named export for this value.
2. Otherwise find the named export the module offers (export it by name if it has none), and import it by that name: `import { name } from "module"`.

## Examples

### JavaScript

#### Don't

```js
// src/parse.js
export function parse() {}
export default parse;
// src/app.js
import parse from "./parse";
parse();
```

#### Do

```js
// src/parse.js
export function parse() {}
export default parse;
// src/app.js
import { parse } from "./parse";
parse();
```

### JSX

#### Don't

```jsx
// src/parse.jsx
export function parse() {}
export default parse;
// src/app.jsx
import parse from "./parse";
parse();
```

#### Do

```jsx
// src/parse.jsx
export function parse() {}
export default parse;
// src/app.jsx
import { parse } from "./parse";
parse();
```

### TypeScript

#### Don't

```ts
// src/parse.ts
export function parse() {}
export default parse;
// src/app.ts
import parse from "./parse";
parse();
```

#### Do

```ts
// src/parse.ts
export function parse() {}
export default parse;
// src/app.ts
import { parse } from "./parse";
parse();
```

### TSX

#### Don't

```tsx
// src/parse.tsx
export function parse() {}
export default parse;
// src/app.tsx
import parse from "./parse";
parse();
```

#### Do

```tsx
// src/parse.tsx
export function parse() {}
export default parse;
// src/app.tsx
import { parse } from "./parse";
parse();
```
