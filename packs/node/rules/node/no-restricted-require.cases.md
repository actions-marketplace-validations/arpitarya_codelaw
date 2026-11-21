# node/no-restricted-require — cases

### case: silent without the option

```js
const _ = require("lodash");
```

```expect
(none)
```

### case: a require

```js options={"modules":["lodash","lodash/*"]}
const _ = require("lodash");
const map = require("lodash.map");
```

```expect
1:19 node/no-restricted-require
```

### case: a computed name is not read

```js options={"modules":["lodash","lodash/*"]}
const _ = require(name);
const m = require(`lodash/${x}`);
```

```expect
(none)
```

### case: import is the other rule's

```js options={"modules":["lodash"]}
import _ from "lodash";
```

```expect
(none)
```
