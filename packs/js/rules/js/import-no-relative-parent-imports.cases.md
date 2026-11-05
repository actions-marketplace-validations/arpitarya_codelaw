# js/import-no-relative-parent-imports — cases

### case: require is not checked by default, dynamic import is

```js
const a = require("../a");
import("../b");
import c from "..";
import d from "./..d";
```

```expect
2:8 js/import-no-relative-parent-imports
3:15 js/import-no-relative-parent-imports
```
