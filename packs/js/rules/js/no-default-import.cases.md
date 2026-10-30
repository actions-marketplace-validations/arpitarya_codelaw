# js/no-default-import — cases

### case: namespace and side-effect imports are fine

```js
import * as path from "node:path";
import "./setup";
```

```expect

```

### case: a default beside named imports

```js
import React, { useState } from "react";
```

```expect
1:1 js/no-default-import
```
