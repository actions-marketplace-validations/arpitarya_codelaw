---
id: js/jsdoc-imports-as-dependencies
title: "Packages named in JSDoc import() types are dependencies"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which package to depend on, and as what, is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/imports-as-dependencies]
since: 0.5.0
---

A package named in an `import('pkg')` type inside a JSDoc tag is in the nearest `package.json`'s `dependencies` or `devDependencies` — its `@types/pkg` when the package ships no types (read from `node_modules/pkg/package.json`; where it is not installed, either spelling will do, which is looser than the plugin, which then insists on `@types/pkg`). Relative paths and Node's built-in modules are not packages. Nothing is checked when no `package.json` is found.

**Why:** A type that points at a package the project does not depend on resolves on the author's machine and nowhere else.

```codelaw
native: js/jsdoc-imports-as-dependencies
```

## How to fix

1. Add the package (or its `@types/` package) to `devDependencies`, or import the type from a package the project depends on.

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "devDependencies": { "typescript": "5.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

#### Do

```js files
// package.json
{ "devDependencies": { "typescript": "5.0.0", "@types/left-pad": "1.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

### JSX

#### Don't

```jsx files
// package.json
{ "devDependencies": { "typescript": "5.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

#### Do

```jsx files
// package.json
{ "devDependencies": { "typescript": "5.0.0", "@types/left-pad": "1.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

### TypeScript

#### Don't

```ts files
// package.json
{ "devDependencies": { "typescript": "5.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

#### Do

```ts files
// package.json
{ "devDependencies": { "typescript": "5.0.0", "@types/left-pad": "1.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

### TSX

#### Don't

```tsx files
// package.json
{ "devDependencies": { "typescript": "5.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```

#### Do

```tsx files
// package.json
{ "devDependencies": { "typescript": "5.0.0", "@types/left-pad": "1.0.0" } }
// src/index.js
/**
 * @type {import('left-pad').Options}
 */
const opts = {};
```
