---
id: node/file-extension-in-import
title: "File extensions written in relative imports"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { mode: { type: "string", default: "always", description: "always: write the extension; never: leave it out" } }
stability: preview
ported_from: [eslint-plugin-n/file-extension-in-import]
since: 0.5.0
---

By default (`always`), an `import`, `export … from` or `import()` of a relative or absolute path that leaves out the extension of the file it names is reported and the extension is added — `/index.js` for a folder; in TypeScript the extension written is the JavaScript one (`.ts` → `.js`). With `mode: "never"` an extension is reported and removed when only one file has that name. A path that names no file, packages and built-ins are left alone; `require()` is not checked.

**Why:** Node's ES module loader does not add extensions, so an import without one fails at run time.

```codelaw
native: node/file-extension-in-import
```

## Examples

### JavaScript

#### Don't

```js files
// src/main.js
import { a } from "./a";
// src/a.js
export const a = 1;
```

#### Do

```js files
// src/main.js
import { a } from "./a.js";
// src/a.js
export const a = 1;
```

#### Fixed

```js files
// src/main.js
import { a } from "./a.js";
// src/a.js
export const a = 1;
```


### JSX

#### Don't

```jsx files
// src/main.jsx
import { a } from "./a";
// src/a.js
export const a = 1;
```

#### Do

```jsx files
// src/main.jsx
import { a } from "./a.js";
// src/a.js
export const a = 1;
```

#### Fixed

```jsx files
// src/main.jsx
import { a } from "./a.js";
// src/a.js
export const a = 1;
```


### TypeScript

#### Don't

```ts files
// src/main.ts
import { a } from "./a";
// src/a.js
export const a = 1;
```

#### Do

```ts files
// src/main.ts
import { a } from "./a.js";
// src/a.js
export const a = 1;
```

#### Fixed

```ts files
// src/main.ts
import { a } from "./a.js";
// src/a.js
export const a = 1;
```


### TSX

#### Don't

```tsx files
// src/main.tsx
import { a } from "./a";
// src/a.js
export const a = 1;
```

#### Do

```tsx files
// src/main.tsx
import { a } from "./a.js";
// src/a.js
export const a = 1;
```

#### Fixed

```tsx files
// src/main.tsx
import { a } from "./a.js";
// src/a.js
export const a = 1;
```
