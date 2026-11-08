---
id: js/import-extensions
title: "No file extension in a relative import that resolves without it"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "Node's ES modules need the extension, so dropping it is right only for code a bundler resolves"
options: { mode: { type: "string", default: "never", description: "never: report an extension the path resolves without; always: report a missing one" } }
stability: preview
ported_from: [eslint-plugin-import/extensions]
since: 0.5.0
---

By default (`never`), a relative import whose path ends in an extension (`./util.js`) that resolves to the same file without it (`./util`, by the resolver's `.mjs`, `.js`, `.json`, `.node`) is reported, and the suggestion drops the extension; `.json` imports keep theirs. With `mode: "always"`, a relative import written without the extension of the file it resolves to is reported instead. Packages and built-ins are left alone.

**Why:** One way of writing paths across the project makes them easy to read and to search.

```codelaw
native: js/import-extensions
```

## Examples

### JavaScript

#### Don't

```js files
// src/app.js
import { util } from "./util.js";
// src/util.js
export const util = 1;
```

#### Do

```js files
// src/app.js
import { util } from "./util";
// src/util.js
export const util = 1;
```

#### Fixed

```js files
// src/app.js
import { util } from "./util";
// src/util.js
export const util = 1;
```


### JSX

#### Don't

```jsx files
// src/app.jsx
import { util } from "./util.js";
// src/util.js
export const util = 1;
```

#### Do

```jsx files
// src/app.jsx
import { util } from "./util";
// src/util.js
export const util = 1;
```

#### Fixed

```jsx files
// src/app.jsx
import { util } from "./util";
// src/util.js
export const util = 1;
```


### TypeScript

#### Don't

```ts files
// src/app.ts
import { util } from "./util.js";
// src/util.js
export const util = 1;
```

#### Do

```ts files
// src/app.ts
import { util } from "./util";
// src/util.js
export const util = 1;
```

#### Fixed

```ts files
// src/app.ts
import { util } from "./util";
// src/util.js
export const util = 1;
```


### TSX

#### Don't

```tsx files
// src/app.tsx
import { util } from "./util.js";
// src/util.js
export const util = 1;
```

#### Do

```tsx files
// src/app.tsx
import { util } from "./util";
// src/util.js
export const util = 1;
```

#### Fixed

```tsx files
// src/app.tsx
import { util } from "./util";
// src/util.js
export const util = 1;
```
