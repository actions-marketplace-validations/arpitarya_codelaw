# js/import-style — cases

### case: util by name

```js
const { format } = require("node:util");
```

```expect
(none)
```

### case: util as a whole

```js
const util = require("util");
```

```expect
1:14 js/import-style
```

### case: an unconfigured module

```js
import { x } from "lodash";
```

```expect
(none)
```
