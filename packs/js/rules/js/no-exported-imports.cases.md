# js/no-exported-imports — cases

### case: a default import

```js
import D from "mod";
export { D };
```

```expect
2:10 js/no-exported-imports
```

### case: the import is also used

```js
import { a } from "mod";
use(a);
export { a };
```

```expect
3:10 js/no-exported-imports
```

### case: a local declaration

```js
const a = 1;
export { a };
```

```expect
(none)
```
