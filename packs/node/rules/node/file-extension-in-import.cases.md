# node/file-extension-in-import — cases

### case: a missing file

```js files
// src/main.js
import m from "./missing";
```

```expect
(none)
```

### case: a folder

```js files
// src/main.js
import d from "./dir";
// src/dir/index.js
export default 1;
```

```expect
src/main.js:1:15 node/file-extension-in-import
```

```fixed
// src/main.js
import d from "./dir/index.js";
// src/dir/index.js
export default 1;
```
