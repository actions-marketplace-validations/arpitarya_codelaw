# js/import-no-useless-path-segments — cases

### case: a double slash

```js files
// src/a.js
import b from "./x//b";
// src/x/b.js
export default 1;
```

```expect
src/a.js:1:15 js/import-no-useless-path-segments
```

```fixed
// src/a.js
import b from "./x/b";
// src/x/b.js
export default 1;
```
