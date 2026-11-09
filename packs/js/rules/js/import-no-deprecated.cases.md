# js/import-no-deprecated — cases

### case: a name that is not marked

```js files
// src/lib.js
export function fresh() {}
// src/app.js
import { fresh } from "./lib";
fresh();
```

```expect
(none)
```

### case: the import and each use

```js files
// src/lib.js
/** @deprecated */
export function old() {}
// src/app.js
import { old } from "./lib";
old();
old();
```

```expect
src/app.js:1:10 js/import-no-deprecated
src/app.js:2:1 js/import-no-deprecated
src/app.js:3:1 js/import-no-deprecated
```

### case: a deprecated default

```js files
// src/lib.js
/** @deprecated use fresh */
export default function legacy() {}
// src/app.js
import legacy from "./lib";
legacy();
```

```expect
src/app.js:1:8 js/import-no-deprecated
src/app.js:2:1 js/import-no-deprecated
```

### case: through a re-export

```js files
// src/lib.js
/** @deprecated */
export const old = 1;
// src/index.js
export { old } from "./lib";
// src/app.js
import { old } from "./index";
console.log(old);
```

```expect
src/app.js:1:10 js/import-no-deprecated
src/app.js:2:13 js/import-no-deprecated
```

### case: a namespace member

```js files
// src/lib.js
/** @deprecated */
export const old = 1;
export const fresh = 2;
// src/app.js
import * as lib from "./lib";
console.log(lib.fresh, lib.old);
```

```expect
src/app.js:2:28 js/import-no-deprecated
```

### case: a comment further up marks nothing

```js files
// src/lib.js
/** @deprecated */

export const far = 1;
// src/app.js
import { far } from "./lib";
console.log(far);
```

```expect
(none)
```
