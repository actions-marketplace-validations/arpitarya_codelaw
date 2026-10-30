# node/global-require — cases

### case: a top-level ternary is left alone

```js
const impl = fast ? require("./fast") : require("./slow");
```

```expect

```

### case: a braceless if is reported

```js
if (debug) require("./trace");
```

```expect
1:12 node/global-require
```
