# js/no-bun-modules — cases

### case: the bun package

```js
import { serve } from "bun";
```

```expect
1:1 js/no-bun-modules
```

### case: a require

```js
const test = require("bun:test");
```

```expect
1:14 js/no-bun-modules
```

### case: a package that merely starts with bun

```js
import bunyan from "bunyan";
```

```expect
(none)
```
