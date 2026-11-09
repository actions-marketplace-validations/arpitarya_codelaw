---
id: js/import-no-useless-path-segments
title: "No useless segments in a relative import path"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { noUselessIndex: { type: "boolean", default: false, description: "also report a trailing /index" }, commonjs: { type: "boolean", default: false, description: "also check require()" } }
stability: preview
ported_from: [eslint-plugin-import/no-useless-path-segments]
since: 0.5.0
---

A relative import path with segments that change nothing — `//`, `/./`, a trailing `/`, `x/..` — or that climbs further than it needs to (`../lib/x` from inside `lib`), and resolves to the same file as the shorter form, is reported and rewritten to it. With `noUselessIndex`, a trailing `/index` is dropped too.

**Why:** The shortest path to a file is the easiest to read and to keep right when files move.

```codelaw
native: js/import-no-useless-path-segments
```

## Examples

### JavaScript

#### Don't

```js files
// src/lib/f.js
import { z } from "../lib/z";
// src/lib/z.js
export const z = 1;
```

#### Do

```js files
// src/lib/f.js
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```

#### Fixed

```js files
// src/lib/f.js
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```


### JSX

#### Don't

```jsx files
// src/lib/f.jsx
import { z } from "../lib/z";
// src/lib/z.js
export const z = 1;
```

#### Do

```jsx files
// src/lib/f.jsx
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```

#### Fixed

```jsx files
// src/lib/f.jsx
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```


### TypeScript

#### Don't

```ts files
// src/lib/f.ts
import { z } from "../lib/z";
// src/lib/z.js
export const z = 1;
```

#### Do

```ts files
// src/lib/f.ts
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```

#### Fixed

```ts files
// src/lib/f.ts
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```


### TSX

#### Don't

```tsx files
// src/lib/f.tsx
import { z } from "../lib/z";
// src/lib/z.js
export const z = 1;
```

#### Do

```tsx files
// src/lib/f.tsx
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```

#### Fixed

```tsx files
// src/lib/f.tsx
import { z } from "./z";
// src/lib/z.js
export const z = 1;
```
