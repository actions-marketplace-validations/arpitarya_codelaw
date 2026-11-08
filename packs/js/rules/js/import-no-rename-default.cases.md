# js/import-no-rename-default — cases

### case: a default that wraps a call

```js files
// src/panel.js
import { connect } from "./store";
function Panel() {}
export default connect(Panel);
// src/store.js
export const connect = (x) => x;
// src/app.js
import Wrapped from "./panel";
console.log(Wrapped);
```

```expect
src/app.js:1:8 js/import-no-rename-default
```

### case: an anonymous default has no name

```js files
// src/lib.js
export default () => 1;
// src/app.js
import anything from "./lib";
console.log(anything);
```

```expect
(none)
```

### case: the same name

```js files
// src/lib.js
function make() {}
export default make;
// src/app.js
import make from "./lib";
console.log(make);
```

```expect
(none)
```

### case: default as an alias

```js files
// src/lib.js
export default class Widget {}
// src/app.js
import { default as Thing } from "./lib";
console.log(Thing);
```

```expect
src/app.js:1:10 js/import-no-rename-default
```

### case: a named import is not a default

```js files
// src/lib.js
export default class Widget {}
export const other = 1;
// src/app.js
import { other as renamed } from "./lib";
console.log(renamed);
```

```expect
(none)
```

### case: a class default and the exported identifier

```js files
// src/lib.js
const thing = {};
export default thing;
// src/app.js
import stuff from "./lib";
console.log(stuff);
```

```expect
src/app.js:1:8 js/import-no-rename-default
```
