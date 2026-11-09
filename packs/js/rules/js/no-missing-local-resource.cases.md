# js/no-missing-local-resource — cases

### case: missing

```js files
// src/load.js
const url = new URL("./nope.png", import.meta.url);
```

```expect
src/load.js:1:21 js/no-missing-local-resource
```
