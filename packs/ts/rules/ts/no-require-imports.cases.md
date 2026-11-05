# ts/no-require-imports — cases

### case: the default choice rewrites a top-level const

```ts
const path = require("node:path");
```

```expect
1:14 ts/no-require-imports
```

```fixed
import path from "node:path";
```

### case: a require inside a function carries no edit

```ts
function load() {
  const path = require("node:path");
  return path;
}
```

```expect
2:16 ts/no-require-imports
```
