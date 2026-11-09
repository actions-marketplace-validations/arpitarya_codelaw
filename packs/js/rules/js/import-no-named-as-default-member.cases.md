# js/import-no-named-as-default-member — cases

### case: a member the module does not export by name

```js files
// src/lib.js
export const find = 1;
export default { find, other: 2 };
// src/app.js
import lib from "./lib";
console.log(lib.other);
```

```expect
(none)
```

### case: the default property

```js files
// src/lib.js
export const find = 1;
export default { find };
// src/app.js
import lib from "./lib";
console.log(lib.default);
```

```expect
(none)
```

### case: a destructuring of the default

```js files
// src/lib.js
export const find = 1;
export default { find };
// src/app.js
import lib from "./lib";
const { find } = lib;
console.log(find);
```

```expect
src/app.js:2:9 js/import-no-named-as-default-member
```

### case: a namespace import is its own thing

```js files
// src/lib.js
export const find = 1;
export default { find };
// src/app.js
import * as lib from "./lib";
console.log(lib.find);
```

```expect
(none)
```

### case: a default passed around whole

```js files
// src/lib.js
export const find = 1;
export default { find };
// src/app.js
import lib from "./lib";
use(lib);
console.log(lib.find);
```

```expect
(none)
```
