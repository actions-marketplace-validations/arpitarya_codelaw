# js/import-no-nodejs-modules — cases

### case: node-prefixed, subpaths and require are reported; look-alike packages are not

```js
import fs from "node:fs";
import { readFile } from "fs/promises";
const cp = require("child_process");
import pathe from "pathe";
import x from "./path";
```

```expect
1:16 js/import-no-nodejs-modules
2:26 js/import-no-nodejs-modules
3:20 js/import-no-nodejs-modules
```
