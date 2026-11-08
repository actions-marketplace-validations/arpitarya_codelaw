# js/import-no-named-as-default — cases

### case: one binding exported twice

```js files
// src/lib.js
function main() {}
export { main };
export default main;
// src/app.js
import main from "./lib";
console.log(main);
```

```expect
(none)
```

### case: a default whose name is also a named export

```js files
// src/lib.js
export const main = 1;
export default function run() {}
// src/app.js
import main from "./lib";
console.log(main);
```

```expect
src/app.js:1:8 js/import-no-named-as-default
```

### case: a module with no default is the default rule's

```js files
// src/lib.js
export const main = 1;
// src/app.js
import main from "./lib";
console.log(main);
```

```expect
(none)
```

### case: the same thing re-exported twice from one module

```js files
// src/inner.js
export const widget = 1;
// src/lib.js
export { widget } from "./inner";
export { widget as default } from "./inner";
// src/app.js
import widget from "./lib";
console.log(widget);
```

```expect
(none)
```

### case: a default import with another name

```js files
// src/lib.js
export const main = 1;
export default function run() {}
// src/app.js
import run from "./lib";
console.log(run);
```

```expect
(none)
```
