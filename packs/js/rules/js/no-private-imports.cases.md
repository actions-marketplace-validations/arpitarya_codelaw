# js/no-private-imports — cases

### case: `@private` and `@package` outside their folders

```js files
// src/lib/impl.js
/** @private */
export const secret = 1;
/** @package */
export const shared = 2;
export const open = 3;
// src/lib/peer.js
import { secret, shared, open } from "./impl";
// src/lib/deep/child.js
import { secret, shared } from "../impl";
// src/app.js
import { secret, shared, open } from "./lib/impl";
```

```expect
src/app.js:1:10 js/no-private-imports
src/app.js:1:18 js/no-private-imports
src/lib/deep/child.js:1:10 js/no-private-imports
```

### case: a default import and a namespace member

```js files
// src/lib/impl.js
/** @package */
export default function main() {}
/** @private */
export const secret = 1;
// src/app.js
import main from "./lib/impl";
import * as impl from "./lib/impl";
impl.secret;
```

```expect
src/app.js:1:8 js/no-private-imports
src/app.js:3:6 js/no-private-imports
```

### case: followed through an index re-export

```js files
// src/lib/impl.js
/** @package */
export const shared = 2;
// src/lib/index.js
export { shared } from "./impl";
import { shared as s } from "./impl";
// src/app.js
import { shared } from "./lib";
```

```expect
src/app.js:1:10 js/no-private-imports
```

### case: a tag is a whole word

```js files
// src/lib/impl.js
/** @packageDocumentation */
export const a = 1;
// src/app.js
import { a } from "./lib/impl";
```

```expect
(none)
```

### case: a dynamic import and a require are not judged

```js files
// src/lib/impl.js
/** @private */
export const secret = 1;
// src/app.js
import("./lib/impl");
require("./lib/impl");
```

```expect
(none)
```
