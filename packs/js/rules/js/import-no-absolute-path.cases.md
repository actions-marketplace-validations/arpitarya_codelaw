# js/import-no-absolute-path — cases

### case: require and dynamic import are checked too

```js
const a = require("/srv/app/a");
import("/srv/app/b");
const c = require("./c");
```

```expect
1:19 js/import-no-absolute-path
2:8 js/import-no-absolute-path
```

### case: a plain string that is not a module source is left alone

```js
const p = "/etc/hosts";
log("/tmp");
```

```expect

```
