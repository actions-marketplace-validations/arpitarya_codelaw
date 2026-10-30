# js/no-redundant-use-strict — cases

### case: a script's own directive is needed

```js
'use strict';
run();
```

```expect

```

### case: a function's directive in a strict script

```js
'use strict';
function f() {
  'use strict';
  return 1;
}
```

```expect
3:3 js/no-redundant-use-strict
```

### case: a function's directive in a sloppy script is needed

```js
function f() {
  'use strict';
  return 1;
}
```

```expect

```
