# js/use-node-assert-strict — cases

### case: the bare module name, single quotes

```js
import assert from 'assert';
```

```expect
1:1 js/use-node-assert-strict
```

### case: already strict

```js
import assert from "node:assert/strict";
```

```expect
(none)
```

### case: a named import

```js
import { equal } from "node:assert";
```

```expect
1:1 js/use-node-assert-strict
```
