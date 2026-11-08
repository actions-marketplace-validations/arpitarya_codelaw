# js/avoid-mutating-nested-properties-of-shallow-clones — cases

### case: one level is fine

```js
import _ from "lodash";
const o = _.clone(src);
o.tag = 1;
```

```expect
(none)
```

### case: isolated first

```js
import _ from "lodash";
const o = _.clone(src);
o.meta = {};
o.meta.tag = "x";
```

```expect
(none)
```

### case: a named import and delete

```js
import { clone } from "lodash-es";
const o = clone(src);
delete o.list.first;
```

```expect
3:1 js/avoid-mutating-nested-properties-of-shallow-clones
```

```fixed
import { clone } from "lodash-es";
const o = structuredClone(src);
delete o.list.first;
```

### case: a local clone

```js
const clone = (x) => x;
const o = clone(src);
o.a.b = 1;
```

```expect
(none)
```
