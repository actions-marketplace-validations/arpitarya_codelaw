---
id: node/no-extraneous-require
title: "Required packages must be declared"
languages: [javascript, jsx, typescript, tsx]
scope: project
fix: suggest
fix_reason: "the package is added to the nearest `package.json` at the version installed (`^<version>` from `node_modules`), as a choice of `dependencies` (the default) or `devDependencies` (the default for a test file); a diff to accept. A package not installed offers no fix: its version is the owner's choice (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [eslint-plugin-n/no-extraneous-require, eslint-plugin-import/no-extraneous-dependencies, eslint-plugin-sonarjs/no-implicit-dependencies]
since: 0.1.0
---

Every package a `require()` names is declared in the nearest `package.json`: `dependencies`, `devDependencies`, `peerDependencies` or `optionalDependencies`.

**Why:** an undeclared package works only while something else happens to install it, and breaks for every user who doesn't have it.

```codelaw
graph:
  import:
    form: [require]
    declared: false
message: "`$PACKAGE` is required but not declared in `$MANIFEST`"
fix:
  do: [add-dependency]
  choices:
    - { label: dependencies }
    - { label: devDependencies }
```

## How to fix

1. Accept the fix where it is offered: `codelaw fix --unsafe --choice dependencies` (or `devDependencies` if only tests and tooling use it) adds the installed version to the nearest `package.json`.
2. Otherwise add it with your package manager's `add` command.

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" } }
// src/app.js
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (dependencies)

```js files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.js
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (devDependencies)

```js files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" }, "devDependencies": { "lodash": "^4.17.21" } }
// src/app.js
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Do

```js files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.js
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

### JSX

#### Don't

```jsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" } }
// src/app.jsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (dependencies)

```jsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.jsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (devDependencies)

```jsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" }, "devDependencies": { "lodash": "^4.17.21" } }
// src/app.jsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Do

```jsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.jsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

### TypeScript

#### Don't

```ts files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" } }
// src/app.ts
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (dependencies)

```ts files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.ts
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (devDependencies)

```ts files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" }, "devDependencies": { "lodash": "^4.17.21" } }
// src/app.ts
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Do

```ts files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.ts
const lodash = require("lodash");
console.log(lodash);
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

### TSX

#### Don't

```tsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" } }
// src/app.tsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (dependencies)

```tsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.tsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Fixed (devDependencies)

```tsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0" }, "devDependencies": { "lodash": "^4.17.21" } }
// src/app.tsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```

#### Do

```tsx files
// package.json
{ "name": "app", "dependencies": { "react": "19.0.0", "lodash": "^4.17.21" } }
// src/app.tsx
const lodash = require("lodash");
export const App = () => <p>{String(lodash)}</p>;
// node_modules/lodash/package.json
{ "name": "lodash", "version": "4.17.21" }
```
