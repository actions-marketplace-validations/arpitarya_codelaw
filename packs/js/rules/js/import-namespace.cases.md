# js/import-namespace — cases

### case: every member exists

```js files
// src/lib.js
export const one = 1;
export default 2;
// src/app.js
import * as lib from "./lib";
console.log(lib.one, lib.default);
```

```expect
(none)
```

### case: a destructured name

```js files
// src/lib.js
export const one = 1;
// src/app.js
import * as lib from "./lib";
const { one, two } = lib;
console.log(one, two);
```

```expect
src/app.js:2:14 js/import-namespace
```

### case: a string key

```js files
// src/lib.js
export const one = 1;
// src/app.js
import * as lib from "./lib";
console.log(lib["one"], lib["two"]);
```

```expect
src/app.js:2:29 js/import-namespace
```

### case: a namespace passed whole is not judged

```js files
// src/lib.js
export const one = 1;
// src/app.js
import * as lib from "./lib";
use(lib);
console.log(lib.missing);
```

```expect
(none)
```

### case: a module that exports nothing

```js files
// src/lib.js
import "./other";
// src/other.js
export const x = 1;
// src/app.js
import * as lib from "./lib";
console.log(lib);
```

```expect
src/app.js:1:8 js/import-namespace
```

### case: names passed on by an export star

```js files
// src/lib.js
export * from "./inner";
// src/inner.js
export const deep = 1;
// src/app.js
import * as lib from "./lib";
console.log(lib.deep, lib.shallow);
```

```expect
src/app.js:2:27 js/import-namespace
```
