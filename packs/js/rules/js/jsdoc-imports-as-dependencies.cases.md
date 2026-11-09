# js/jsdoc-imports-as-dependencies — cases

### case: a package that is not a dependency

```js files
// package.json
{ "dependencies": { "foo": "1.0.0" } }
// src/a.js
/**
 * @type {import('bar').Thing}
 */
const x = 1;
```

```expect
src/a.js:2:4 js/jsdoc-imports-as-dependencies
```

### case: a dependency

```js files
// package.json
{ "dependencies": { "bar": "1.0.0" } }
// src/a.js
/**
 * @type {import('bar').Thing}
 */
const x = 1;
```

```expect
(none)
```

### case: a scoped package and a subpath

```js files
// package.json
{ "devDependencies": { "@scope/pkg": "1.0.0" } }
// src/a.js
/**
 * @type {import('@scope/pkg/lib').Thing}
 */
const x = 1;
```

```expect
(none)
```

### case: a relative path and a built-in module

```js files
// package.json
{ "dependencies": {} }
// src/a.js
/**
 * @param {import('./local').A} a
 * @param {import('node:fs').Stats} b
 * @param {import('path').ParsedPath} c
 */
function f(a, b, c) {}
```

```expect
(none)
```

### case: the types package is a dependency

```js files
// package.json
{ "devDependencies": { "@types/bar": "1.0.0" } }
// src/a.js
/**
 * @type {import('bar').Thing}
 */
const x = 1;
```

```expect
(none)
```
