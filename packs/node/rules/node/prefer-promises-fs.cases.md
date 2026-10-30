# node/prefer-promises-fs — cases

### case: the promise module and sync calls are left alone

```js
const fs = require("fs");
fs.promises.readFile(path);
fs.readFileSync(path);
```

```expect

```
