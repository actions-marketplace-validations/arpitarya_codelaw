# js/no-implicit-globals — cases

### case: silent without the option

```js
var counter = 0;
function bump() {}
leaked = 1;
```

```expect
(none)
```

### case: var, function and a leak

```js options={"script":true}
var counter = 0;
function bump() {
  counter += 1;
}
leaked = 1;
```

```expect
1:5 js/no-implicit-globals
2:1 js/no-implicit-globals
5:1 js/no-implicit-globals
```

### case: a module is not a script

```js options={"script":true}
import { a } from "./a";
var counter = a;
```

```expect
(none)
```

### case: let, const and class need the option

```js options={"script":true}
let a = 1;
const b = 2;
class C {}
```

```expect
(none)
```

### case: lexical bindings

```js options={"script":true,"lexicalBindings":true}
let a = 1;
const b = 2;
class C {}
```

```expect
1:5 js/no-implicit-globals
2:7 js/no-implicit-globals
3:1 js/no-implicit-globals
```

### case: a var in a block is global too

```js options={"script":true}
if (ready) {
  var counter = 0;
}
```

```expect
2:7 js/no-implicit-globals
```

### case: inside a function is local

```js options={"script":true}
(function () {
  var counter = 0;
  function bump() {}
})();
```

```expect
(none)
```

### case: exported

```js options={"script":true}
/* exported counter */
var counter = 0;
var other = 1;
```

```expect
3:5 js/no-implicit-globals
```

### case: a destructured var

```js options={"script":true}
var { a, b } = source;
```

```expect
1:7 js/no-implicit-globals
1:10 js/no-implicit-globals
```
