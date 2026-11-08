# js/import-no-cycle — cases

### case: a chain of three

```js files
// src/a.js
import { b } from "./b";
export const a = b;
// src/b.js
import { c } from "./c";
export const b = c;
// src/c.js
import { a } from "./a";
export const c = a;
```

```expect
src/a.js:1:19 js/import-no-cycle
src/b.js:1:19 js/import-no-cycle
src/c.js:1:19 js/import-no-cycle
```

### case: a type-only import closes nothing

```ts files
// src/a.ts
import type { B } from "./b";
export type A = B;
// src/b.ts
import { value } from "./a";
export type B = typeof value;
```

```expect
(none)
```

### case: a dynamic import is a way back

```js files
// src/a.js
export const load = () => import("./b");
// src/b.js
import { load } from "./a";
export const b = load;
```

```expect
src/a.js:1:34 js/import-no-cycle
src/b.js:1:22 js/import-no-cycle
```

### case: a one-way chain

```js files
// src/a.js
import { b } from "./b";
export const a = b;
// src/b.js
import { c } from "./c";
export const b = c;
// src/c.js
export const c = 1;
```

```expect
(none)
```

### case: a re-export is an edge

```js files
// src/a.js
export { b } from "./b";
// src/b.js
import "./a";
export const b = 1;
```

```expect
src/a.js:1:19 js/import-no-cycle
src/b.js:1:8 js/import-no-cycle
```

### case: the import of an external package

```js files
// src/a.js
import x from "somewhere";
export default x;
```

```expect
(none)
```
