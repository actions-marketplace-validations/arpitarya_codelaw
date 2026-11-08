---
id: node/prefer-process-get-builtin-module
title: "`process.getBuiltinModule()` to load a built-in module"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the call no longer goes through the module loader, which a bundler or a test mock may rely on"
options: { version: { type: "string", default: "", description: "the Node versions the code runs on, as a semver range; by default package.json's engines.node" } }
stability: preview
ported_from: [eslint-plugin-n/prefer-process-get-builtin-module]
since: 0.5.0
---

When every Node version the project supports has `process.getBuiltinModule()` (the `version` option or package.json's `engines.node` is within `^20.16.0 || >=22.3.0`), `require("<built-in>")` with the global `require` and `await import("<built-in>")` are reported. The suggestion rewrites the `require()`; an `import()` gives a namespace, so it is left to the author. With no Node range, or one reaching older versions, the rule does nothing, as upstream.

**Why:** `process.getBuiltinModule()` loads a built-in synchronously from either kind of module and can't pick up a package of the same name.

```codelaw
native: node/prefer-process-get-builtin-module
```

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.js
const path = require("node:path");
```

#### Do

```js files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.js
const path = process.getBuiltinModule("node:path");
```

#### Fixed

```js files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.js
const path = process.getBuiltinModule("node:path");
```


### JSX

#### Don't

```jsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.jsx
const path = require("node:path");
```

#### Do

```jsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.jsx
const path = process.getBuiltinModule("node:path");
```

#### Fixed

```jsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.jsx
const path = process.getBuiltinModule("node:path");
```


### TypeScript

#### Don't

```ts files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.ts
const path = require("node:path");
```

#### Do

```ts files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.ts
const path = process.getBuiltinModule("node:path");
```

#### Fixed

```ts files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.ts
const path = process.getBuiltinModule("node:path");
```


### TSX

#### Don't

```tsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.tsx
const path = require("node:path");
```

#### Do

```tsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.tsx
const path = process.getBuiltinModule("node:path");
```

#### Fixed

```tsx files
// package.json
{ "name": "app", "engines": { "node": ">=22.3.0" } }
// src/a.tsx
const path = process.getBuiltinModule("node:path");
```
