---
id: js/import-no-named-as-default
title: "A default import must not take the name of a named export"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: manual
fix_reason: "the binding needs a name of the author's choosing for the default export; the code holds only the named export's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import/no-named-as-default, eslint-plugin-import-x/no-named-as-default]
since: 0.5.0
---

`import x from "./m"` where `./m` has a default export and also exports `x` by name is reported at the default specifier: it is usually the named export that was meant (`import { x }`). Where the two exports are one binding — `export { x }` beside `export default x`, or two re-exports of the same name from the same module — nothing is reported; upstream cannot tell a local binding exported twice from two, and reports it. A module with no default export is the `js/import-default` rule's.

**Why:** `import helper from` reads like the named `helper` and is the module's default, which is a different value; the code runs and does the wrong thing.

```codelaw
graph:
  import:
    default_name: named
message: "`$NAME` is a named export of `$SOURCE`; the default import takes that name"
```

## How to fix

1. If the named export was meant, import it: `import { helper } from "./lib"`.
2. If the default was meant, give the binding a name that is not one of the module's named exports.

## Examples

### JavaScript

#### Don't

```js files
// src/lib.js
export const helper = () => 1;
export default function main() {}
// src/app.js
import helper from "./lib";
console.log(helper());
```

#### Do

```js files
// src/lib.js
export const helper = () => 1;
export default function main() {}
// src/app.js
import main, { helper } from "./lib";
console.log(main, helper());
```

### JSX

#### Don't

```jsx files
// src/lib.jsx
export const helper = () => 1;
export default function main() {}
// src/app.jsx
import helper from "./lib";
console.log(helper());
```

#### Do

```jsx files
// src/lib.jsx
export const helper = () => 1;
export default function main() {}
// src/app.jsx
import main, { helper } from "./lib";
console.log(main, helper());
```

### TypeScript

#### Don't

```ts files
// src/lib.ts
export const helper = () => 1;
export default function main() {}
// src/app.ts
import helper from "./lib";
console.log(helper());
```

#### Do

```ts files
// src/lib.ts
export const helper = () => 1;
export default function main() {}
// src/app.ts
import main, { helper } from "./lib";
console.log(main, helper());
```

### TSX

#### Don't

```tsx files
// src/lib.tsx
export const helper = () => 1;
export default function main() {}
// src/app.tsx
import helper from "./lib";
console.log(helper());
```

#### Do

```tsx files
// src/lib.tsx
export const helper = () => 1;
export default function main() {}
// src/app.tsx
import main, { helper } from "./lib";
console.log(main, helper());
```
