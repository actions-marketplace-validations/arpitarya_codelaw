# node/prefer-process-get-builtin-module — cases

### case: older Node allowed

```js files
// package.json
{ "name": "app", "engines": { "node": ">=18" } }
// src/a.js
const fs = require("fs");
```

```expect
(none)
```
