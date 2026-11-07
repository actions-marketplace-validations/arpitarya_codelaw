### case: a name both imports hold is kept once

```js
import { a } from "./util";
import { a, b } from "./util";
```

```expect
2:1 js/no-duplicate-imports
```

```fixed
import { a, b } from "./util";
```

### case: an import between them is left where it is

```js
import { a } from "./util";
import x from "./other";
import { b } from "./util";
```

```expect
3:1 js/no-duplicate-imports
```

```fixed
import { a, b } from "./util";
import x from "./other";
```

### case: a default import is merged by hand

```js
import a from "./util";
import { b } from "./util";
```

```expect
2:1 js/no-duplicate-imports
```
