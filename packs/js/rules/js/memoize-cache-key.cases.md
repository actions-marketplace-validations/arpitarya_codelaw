# js/memoize-cache-key — cases

### case: one parameter

```js
import _ from "lodash";
const twice = _.memoize((x) => x * 2);
```

```expect
(none)
```

### case: a per-method package and rest

```js
import memoize from "lodash/memoize";
const j = memoize((...xs) => xs.join());
```

```expect
2:11 js/memoize-cache-key
```
