# js/import-first — cases

### case: a directive and comments before the imports are fine

```js
"use strict";
// header
import { a } from "./a";
a();
```

```expect

```

### case: two late imports keep their order

```js
import a from "./a";
a();
import c from "./c";
c();
import b from "./b";
```

```expect
3:1 js/import-first
5:1 js/import-first
```

```fixed
import a from "./a";
import c from "./c";
import b from "./b";
a();
c();
```
