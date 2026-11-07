# node/no-deprecated-api — cases

### case: a member of a whole-module import, the module read nowhere else

```js
import util from "util";
export const ok = util.isArray([]);
```

```expect
2:19 node/no-deprecated-api
```

```fixed
export const ok = Array.isArray([]);
```

### case: an alias of the same module keeps the import

```js
const os = require("os");
export const dir = os.tmpDir();
```

```expect
2:20 node/no-deprecated-api
```

```fixed
const os = require("os");
export const dir = os.tmpdir();
```

### case: a named alias is imported under its current name

```js
import { tmpDir } from "os";
export const dir = tmpDir();
```

```expect
1:10 node/no-deprecated-api
```

```fixed
import { tmpdir as tmpDir } from "os";
export const dir = tmpDir();
```

### case: a destructured require beside another binding

```js
const { isArray, inspect } = require("util");
export const ok = isArray([]) && inspect;
```

```expect
1:9 node/no-deprecated-api
```

```fixed
const { inspect } = require("util");
export const ok = Array.isArray([]) && inspect;
```

### case: an API whose replacement changes the call stays manual

```js
const url = require("url");
export const u = url.parse("x");
```

```expect
2:18 node/no-deprecated-api
```
