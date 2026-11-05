# node/prefer-node-protocol — cases

### case: packages and relative paths are left alone

```js
import x from "./fs";
import y from "fs-extra";
const z = require("lodash");
```

```expect

```

### case: dynamic import and re-export

```js
export * from "os";
const m = await import("crypto");
```

```expect
1:16 node/prefer-node-protocol
2:25 node/prefer-node-protocol
```

```fixed
export * from "node:os";
const m = await import("node:crypto");
```
