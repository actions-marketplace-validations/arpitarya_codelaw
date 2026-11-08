---
id: js/import-no-rename-default
title: "A default import should go by the default export's own name"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "the right name is the one the default export has in its own file; renaming a binding is left to the author because other names in the importing file may already use it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import-x/no-rename-default]
since: 0.5.0
---

A default import (`import x from "./m"`, or `import { default as x } from "./m"`) bound to a name other than the one the default export has in `./m` is reported. That name is the function or class declared (`export default function make() {}`), the identifier exported (`export default widget`), the first argument of the call it wraps (`export default connect(Panel)` is `Panel`), or the local of `export { item as default }`. An anonymous default export, a default that is re-exported from elsewhere, and a module that is not an ES module have no name to compare. Upstream's `commonjs` and `preventRenamingBindings` options are not built; the rule reads as upstream's defaults, except that `require()` is not checked.

**Why:** a default export has no name at the import, so the name each importer picks is the only name the thing has; one module imported under three names is three names to search for.

```codelaw
graph:
  import:
    default_name: differs
message: "`$SOURCE` has a default export `$NAME`; it is imported as `$LOCAL`"
```

## How to fix

1. Rename the binding to the name the message gives, and every use of it.
2. Or rename the default export to the name the importers use, in its own file.

## Examples

### JavaScript

#### Don't

```js files
// src/make.js
export default function makeWidget() {}
// src/app.js
import build from "./make";
console.log(build());
```

#### Do

```js files
// src/make.js
export default function makeWidget() {}
// src/app.js
import makeWidget from "./make";
console.log(makeWidget());
```

### JSX

#### Don't

```jsx files
// src/make.jsx
export default function makeWidget() {}
// src/app.jsx
import build from "./make";
console.log(build());
```

#### Do

```jsx files
// src/make.jsx
export default function makeWidget() {}
// src/app.jsx
import makeWidget from "./make";
console.log(makeWidget());
```

### TypeScript

#### Don't

```ts files
// src/make.ts
export default function makeWidget() {}
// src/app.ts
import build from "./make";
console.log(build());
```

#### Do

```ts files
// src/make.ts
export default function makeWidget() {}
// src/app.ts
import makeWidget from "./make";
console.log(makeWidget());
```

### TSX

#### Don't

```tsx files
// src/make.tsx
export default function makeWidget() {}
// src/app.tsx
import build from "./make";
console.log(build());
```

#### Do

```tsx files
// src/make.tsx
export default function makeWidget() {}
// src/app.tsx
import makeWidget from "./make";
console.log(makeWidget());
```
