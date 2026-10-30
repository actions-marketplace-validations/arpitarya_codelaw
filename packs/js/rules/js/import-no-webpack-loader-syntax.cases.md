# js/import-no-webpack-loader-syntax — cases

### case: require with loaders is reported too

```js
const raw = require("raw-loader!./notes.txt");
```

```expect
1:21 js/import-no-webpack-loader-syntax
```
