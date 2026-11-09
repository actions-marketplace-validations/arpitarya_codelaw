# js/prefer-export-from — cases

### case: a default import

```js
import d from "./m.js";
export default d;
```

```expect
2:1 js/prefer-export-from
```

```fixed
export { default } from "./m.js";
```

### case: a namespace exported as default

```js
import * as ns from "./m.js";
export default ns;
```

```expect
(none)
```

### case: documented

```js
import { a } from "./m.js";
/** The a. */
export const b = a;
```

```expect
(none)
```

### case: also used here

```js
import { a } from "./m.js";
use(a);
export { a };
```

```expect
3:10 js/prefer-export-from
```

```fixed
import { a } from "./m.js";
use(a);
export { a } from "./m.js";
```
