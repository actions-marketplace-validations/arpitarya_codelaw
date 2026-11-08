# js/import-no-unused-modules — cases

### case: a namespace read names what it reads

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/space.js
export const read = 1;
export const unread = 2;
// src/main.js
import * as space from "./space";
export const run = () => space.read;
```

```expect
src/space.js:2:14 js/import-no-unused-modules
```

### case: a namespace passed whole reads everything

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/space.js
export const read = 1;
export const unread = 2;
// src/main.js
import * as space from "./space";
export const run = () => use(space);
```

```expect
(none)
```

### case: a module no entry reads is a module with no exports

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/main.js
import "./side";
export const run = 1;
// src/side.js
console.log("effect");
```

```expect
src/side.js:1:1 js/import-no-unused-modules
```

### case: a default export nobody imports

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const used = 1;
export default 2;
// src/main.js
import { used } from "./lib";
export const run = () => used;
```

```expect
src/lib.js:2:1 js/import-no-unused-modules
```

### case: a barrel passes its names on once it is read

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/barrel.js
export * from "./lib";
// src/lib.js
export const a = 1;
// src/main.js
import { a } from "./barrel";
export const run = () => a;
```

```expect
(none)
```

### case: a literal require reads every name

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const a = 1;
export const b = 2;
// src/main.js
const lib = require("./lib");
export const run = () => lib;
```

```expect
(none)
```

### case: an unused export still used in its own file loses only its keyword

```js files
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const used = 1;
export const local = used + 1;
export const again = () => local;
// src/main.js
import { used, again } from "./lib";
export const run = () => [used, again];
```

```expect
src/lib.js:2:14 js/import-no-unused-modules
```

```fixed
// package.json
{ "name": "app", "private": true, "main": "src/main.js" }
// src/lib.js
export const used = 1;
const local = used + 1;
export const again = () => local;
// src/main.js
import { used, again } from "./lib";
export const run = () => [used, again];
```
