# js/import-prefer-namespace-import — cases

### case: silent without the option

```js
import React from "react";
```

```expect
(none)
```

### case: a default import of a listed module

```js options={"patterns":["react"]}
import React from "react";
```

```expect
1:1 js/import-prefer-namespace-import
```

```fixed
import * as React from "react";
```

### case: default and named imports are split

```js options={"patterns":["react"]}
import React, { useState } from "react";
```

```expect
1:8 js/import-prefer-namespace-import
```

```fixed
import * as React from "react";
import { useState } from "react";
```

### case: a regular expression

```js options={"patterns":["/^react(-dom)?$/"]}
import ReactDOM from "react-dom";
import other from "lodash";
```

```expect
1:1 js/import-prefer-namespace-import
```

```fixed
import * as ReactDOM from "react-dom";
import other from "lodash";
```

### case: a namespace import is fine

```js options={"patterns":["react"]}
import * as React from "react";
```

```expect
(none)
```

### case: a module that is not listed

```js options={"patterns":["react"]}
import debounce from "lodash";
```

```expect
(none)
```
