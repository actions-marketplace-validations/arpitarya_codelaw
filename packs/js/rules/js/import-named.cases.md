# js/import-named — cases

### case: a name behind an export star

```js files
// src/lib.js
export * from "./more";
// src/more.js
export const deep = 1;
// src/app.js
import { deep } from "./lib";
console.log(deep);
```

```expect
(none)
```

### case: a renamed export is known by its new name only

```js files
// src/lib.js
const inner = 1;
export { inner as outer };
// src/app.js
import { outer, inner } from "./lib";
console.log(outer, inner);
```

```expect
src/app.js:1:17 js/import-named
```

### case: a default export can be named default

```js files
// src/lib.js
export default 1;
// src/app.js
import { default as one } from "./lib";
console.log(one);
```

```expect
(none)
```

### case: a CommonJS module lists no names

```js files
// src/lib.js
module.exports = { a: 1 };
// src/app.js
import { a } from "./lib";
console.log(a);
```

```expect
(none)
```

### case: a package outside the project

```js files
// src/app.js
import { merge } from "lodash";
console.log(merge);
```

```expect
(none)
```

### case: an export star that cannot be read hides the names

```js files
// src/lib.js
export * from "somewhere-else";
// src/app.js
import { anything } from "./lib";
console.log(anything);
```

```expect
(none)
```

### case: a type import is erased

```ts files
// src/lib.ts
export const value = 1;
// src/app.ts
import type { Gone } from "./lib";
export type T = Gone;
```

```expect
(none)
```

### case: a re-export of a name the module lacks

```js files
// src/lib.js
export const here = 1;
// src/index.js
export { gone } from "./lib";
```

```expect
src/index.js:1:1 js/import-named
```

### case: a name near nothing offers no fix

```js files
// src/lib.js
export const alpha = 1;
// src/app.js
import { zzzzzz } from "./lib";
console.log(zzzzzz);
```

```expect
src/app.js:1:10 js/import-named
```

### case: the local name survives the fix

```js files
// src/lib.js
export const alpha = 1;
// src/app.js
import { alphx } from "./lib";
console.log(alphx);
```

```expect
src/app.js:1:10 js/import-named
```

```fixed
// src/lib.js
export const alpha = 1;
// src/app.js
import { alpha as alphx } from "./lib";
console.log(alphx);
```
