# js/import-no-restricted-paths — cases

### case: silent without the option

```js
// client/app.js
import { secret } from "../server/secret";
```

```expect
(none)
```

### case: a client file imports server code

```js options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import { secret } from "../server/secret";
```

```expect
2:25 js/import-no-restricted-paths
```

### case: the other direction is fine

```js options={"zones":{"client":{"target":"client","from":"server"}}}
// server/job.js
import { api } from "../client/api";
```

```expect
(none)
```

### case: an except path

```js options={"zones":{"client":{"target":"client","from":"server","except":["config.js","config"]}}}
// client/app.js
import { config } from "../server/config";
import { secret } from "../server/secret";
```

```expect
3:25 js/import-no-restricted-paths
```

### case: a message

```js options={"zones":{"client":{"target":"client","from":"server","message":"Use the API."}}}
// client/app.js
const s = require("../server/secret");
```

```expect
2:19 js/import-no-restricted-paths
```

### case: a glob target

```js options={"zones":{"ui":{"target":"packages/ui/**","from":"packages/core"}}}
// packages/ui/a.js
import x from "../core/x";
```

```expect
2:15 js/import-no-restricted-paths
```

### case: a package is not resolved

```js options={"zones":{"client":{"target":"client","from":"server"}}}
// client/app.js
import React from "react";
```

```expect
(none)
```
