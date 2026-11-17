# node/no-restricted-import — cases

### case: silent without the option

```js
import _ from "lodash";
```

```expect
(none)
```

### case: an import

```js options={"modules":["lodash","lodash/*"]}
import _ from "lodash";
import map from "lodash-es/map";
```

```expect
1:15 node/no-restricted-import
```

### case: a subpath

```js options={"modules":["lodash","lodash/*"]}
import map from "lodash/map";
```

```expect
1:17 node/no-restricted-import
```

### case: export from and import()

```js options={"modules":["lodash","lodash/*"]}
export { map } from "lodash/map";
const m = import("lodash");
```

```expect
1:21 node/no-restricted-import
2:18 node/no-restricted-import
```

### case: a name taken back out

```js options={"modules":["lodash/*","!lodash/map"]}
import a from "lodash/map";
import b from "lodash/get";
```

```expect
2:15 node/no-restricted-import
```

### case: a message

```js options={"modules":["fs"],"messages":{"fs":"Use\u0020node:fs."}}
import fs from "fs";
```

```expect
1:16 node/no-restricted-import
```

### case: require is the other rule's

```js options={"modules":["lodash"]}
const _ = require("lodash");
```

```expect
(none)
```
