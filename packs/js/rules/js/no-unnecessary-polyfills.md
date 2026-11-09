---
id: js/no-unnecessary-polyfills
title: "No polyfill for a feature every target already has"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a bare import of a polyfill (`import \"core-js/…\"`, a bare `require()`) is deleted as a diff to accept; one that binds a name (`import flat from \"array.prototype.flat\"`) is used in the code, and replacing its uses with the built-in is the author's rewrite"
options: { targets: { type: "object", default: {}, description: "the engines to support, as { chrome: \"100\", node: \"20\" }; empty reads the nearest browserslist (`package.json` or `.browserslistrc`, queries like `chrome >= 100`), else `engines` of the nearest package.json" } }
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-polyfills]
since: 0.5.0
---

An import or `require()` of a polyfill — a core-js entry (`core-js/es/array/flat`, `core-js/stable/…`, `core-js-pure/…`), or a package named for the feature it adds (`array.prototype.flat`, `es6-promise`, `polyfill-array-includes`, `p-finally`) — is reported when every engine the project supports already has the feature built in. A core-js entry that stands for several features is reported only when all of them are built in everywhere. A relative import is never a polyfill.

**The targets** are the rule's `targets` option, else the nearest `browserslist` (`package.json` or `.browserslistrc`), else `engines` of the nearest `package.json`. Only queries that spell versions out are read (`chrome >= 100`, `node 18`, `safari >= 15.4`); a query that needs a browser-usage database (`defaults`, `last 2 versions`, `> 0.5%`) leaves the targets unknown, and the rule reports nothing rather than guess. An engine the table does not know counts as one that lacks every feature.

**The table** is the data package `@codelaw/data-polyfills` / `codelaw-data-polyfills` (generated from core-js-compat's data, MIT; L6 decision 15), read only when this rule is on and a file imports a package. Without the package the rule reports nothing. Upstream's resolution of browserslist queries, and its `targets` option as a query string or array, are not built: the option is the targets object.

**Why:** a polyfill for a feature the supported engines have is code every user downloads and runs for nothing, and a reason to keep an old build setting.

```codelaw
native: js/no-unnecessary-polyfills
```

## How to fix

1. Accept the fix where one is offered: `codelaw fix --unsafe` deletes the bare import.
2. Otherwise use the built-in where the polyfilled name is used, and remove the import and the dependency.
3. If an older engine is still supported, list it in the targets: the polyfill is then needed.

## Examples

### JavaScript

#### Don't

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
import "core-js/stable/array/flat";
console.log([1, [2]].flat());
```

#### Do

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
console.log([1, [2]].flat());
```

### JSX

#### Don't

```jsx files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.jsx
import "core-js/stable/array/flat";
console.log([1, [2]].flat());
```

#### Do

```jsx files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.jsx
console.log([1, [2]].flat());
```

### TypeScript

#### Don't

```ts files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.ts
import "core-js/stable/array/flat";
console.log([1, [2]].flat());
```

#### Do

```ts files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.ts
console.log([1, [2]].flat());
```

### TSX

#### Don't

```tsx files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.tsx
import "core-js/stable/array/flat";
console.log([1, [2]].flat());
```

#### Do

```tsx files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.tsx
console.log([1, [2]].flat());
```
