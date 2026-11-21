# js/use-json-import-attributes — cases

### case: a JSON import without the attribute

```js
import data from './data.json';
```

```expect
1:1 js/use-json-import-attributes
```

```fixed
import data from './data.json' with { type: "json" };
```

### case: with the attribute

```js
import data from "./data.json" with { type: "json" };
```

```expect
(none)
```

### case: a script

```js
import run from "./run.js";
```

```expect
(none)
```
