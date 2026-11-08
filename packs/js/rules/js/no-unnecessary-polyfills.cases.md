# js/no-unnecessary-polyfills — cases

### case: a package named for the feature

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
import flat from "array.prototype.flat";
console.log(flat([1, [2]]));
```

```expect
src/app.js:1:18 js/no-unnecessary-polyfills
```

### case: an older engine still needs it

```js files
// package.json
{ "name": "app", "engines": { "node": ">=8" } }
// src/app.js
import "core-js/es/array/flat";
```

```expect
(none)
```

### case: browserslist queries that spell versions out

```js files
// package.json
{ "name": "app", "browserslist": ["chrome >= 100", "safari >= 15.4", "firefox >= 100"] }
// src/app.js
import "core-js/es/array/flat";
```

```expect
src/app.js:1:8 js/no-unnecessary-polyfills
```

```fixed
// package.json
{ "name": "app", "browserslist": ["chrome >= 100", "safari >= 15.4", "firefox >= 100"] }
// src/app.js
```

### case: one engine below the feature keeps the polyfill

```js files
// package.json
{ "name": "app", "browserslist": ["chrome >= 100", "safari >= 11"] }
// src/app.js
import "core-js/es/array/flat";
```

```expect
(none)
```

### case: a query that needs a usage database is not guessed

```js files
// package.json
{ "name": "app", "browserslist": ["defaults"] }
// src/app.js
import "core-js/es/array/flat";
```

```expect
(none)
```

### case: no targets, no verdict

```js files
// package.json
{ "name": "app" }
// src/app.js
import "core-js/es/array/flat";
```

```expect
(none)
```

### case: the entry for everything is needed somewhere

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
import "core-js/stable";
```

```expect
(none)
```

### case: a relative import is not a polyfill

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
import "./array-flat";
```

```expect
(none)
```

### case: a bare require is deleted

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
require("core-js/es/object/assign");
module.exports = {};
```

```expect
src/app.js:1:9 js/no-unnecessary-polyfills
```

```fixed
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
module.exports = {};
```

### case: a legacy alias of one built-in

```js files
// package.json
{ "name": "app", "engines": { "node": ">=20" } }
// src/app.js
import Promise from "es6-promise";
console.log(Promise);
```

```expect
src/app.js:1:21 js/no-unnecessary-polyfills
```
