---
id: js/import-no-relative-packages
title: "No relative import into another package of the repository"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { commonjs: { type: "boolean", default: false, description: "also check require()" } }
stability: preview
ported_from: [eslint-plugin-import/no-relative-packages]
since: 0.5.0
---

A relative import that resolves to a file of another package — one whose nearest named `package.json` differs from the importing file's — is reported and rewritten as an import of that package by its name, with the rest of the path.

**Why:** A relative path across packages skips the package's entry and its declared dependency, so it breaks once the packages are published or built separately.

```codelaw
native: js/import-no-relative-packages
```

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "app" }
// src/x.js
import { a } from "../packages/utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Do

```js files
// package.json
{ "name": "app" }
// src/x.js
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Fixed

```js files
// package.json
{ "name": "app" }
// src/x.js
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```


### JSX

#### Don't

```jsx files
// package.json
{ "name": "app" }
// src/x.jsx
import { a } from "../packages/utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Do

```jsx files
// package.json
{ "name": "app" }
// src/x.jsx
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Fixed

```jsx files
// package.json
{ "name": "app" }
// src/x.jsx
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```


### TypeScript

#### Don't

```ts files
// package.json
{ "name": "app" }
// src/x.ts
import { a } from "../packages/utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Do

```ts files
// package.json
{ "name": "app" }
// src/x.ts
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Fixed

```ts files
// package.json
{ "name": "app" }
// src/x.ts
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```


### TSX

#### Don't

```tsx files
// package.json
{ "name": "app" }
// src/x.tsx
import { a } from "../packages/utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Do

```tsx files
// package.json
{ "name": "app" }
// src/x.tsx
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```

#### Fixed

```tsx files
// package.json
{ "name": "app" }
// src/x.tsx
import { a } from "utils/lib/a";
// packages/utils/package.json
{ "name": "utils" }
// packages/utils/lib/a.js
export const a = 1;
```
