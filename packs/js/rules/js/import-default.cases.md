# js/import-default — cases

### case: a default declared beside names

```js files
// src/lib.js
export const a = 1;
export default function main() {}
// src/app.js
import main from "./lib";
console.log(main);
```

```expect
(none)
```

### case: an export star does not pass the default on

```js files
// src/lib.js
export * from "./inner";
// src/inner.js
export default 1;
// src/app.js
import inner from "./lib";
console.log(inner);
```

```expect
src/app.js:1:8 js/import-default
```

### case: a default re-exported by name

```js files
// src/lib.js
export { default } from "./inner";
// src/inner.js
export default 1;
// src/app.js
import inner from "./lib";
console.log(inner);
```

```expect
(none)
```

### case: a CommonJS module has no ES default to check

```js files
// src/lib.js
module.exports = function () {};
// src/app.js
import lib from "./lib";
console.log(lib);
```

```expect
(none)
```

### case: a default import beside names

```js files
// src/lib.js
export const a = 1;
// src/app.js
import lib, { a } from "./lib";
console.log(lib, a);
```

```expect
src/app.js:1:8 js/import-default
```
