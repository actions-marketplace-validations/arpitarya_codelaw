# node/prefer-global-buffer — cases

### case: an import of Buffer

```js
import { Buffer } from "node:buffer";
Buffer.alloc(1);
```

```expect
1:1 node/prefer-global-buffer
```

### case: a member form

```js
const Buffer = require("buffer").Buffer;
```

```expect
1:1 node/prefer-global-buffer
```

### case: other names from the module are left alone

```js
const { Buffer, constants } = require("buffer");
import { Buffer as B } from "buffer";
```

```expect

```
