# js/jsdoc-no-undefined-types — cases

### case: a type nothing defines

```js
/**
 * @param {Foo} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-no-undefined-types
```

### case: a class in the file

```js
class Foo {}
/**
 * @param {Foo} a
 * @param {Foo[]} b
 * @returns {Promise<Foo>}
 */
function f(a, b) {}
```

```expect
(none)
```

### case: a typedef and a callback

```js
/**
 * @typedef {object} Opts
 */
/**
 * @callback Done
 */
/**
 * @param {Opts} o
 * @param {Done} d
 */
function f(o, d) {}
```

```expect
(none)
```

### case: an import

```js
import { Foo } from './foo.js';
/**
 * @param {Foo} a
 */
function f(a) {}
```

```expect
(none)
```

### case: built-in types

```js
/**
 * @param {Partial<Record<string, number>>} a
 * @param {Map<string, Set<number>>} b
 * @param {Awaited<Promise<string>>} c
 * @param {?string} d
 * @param {Array.<Object>} e
 */
function f(a, b, c, d, e) {}
```

```expect
(none)
```

### case: a template name

```js
/**
 * @template T
 * @param {T} a
 */
function f(a) {}
```

```expect
(none)
```

### case: a type parameter of a function type

```js
/**
 * @param {<T>(x: T) => T} fn
 */
function f(fn) {}
```

```expect
(none)
```

### case: a dotted name whose root is defined

```js
import * as ns from './ns.js';
/**
 * @param {ns.Thing} a
 */
function f(a) {}
```

```expect
(none)
```

### case: a dotted name whose root is not

```js
/**
 * @param {ns.Thing} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-no-undefined-types
```

### case: a globals comment

```js
/* globals Foo */
/**
 * @param {Foo} a
 */
function f(a) {}
```

```expect
(none)
```

### case: defined_types

```js options={"defined_types":["Foo"]}
/**
 * @param {Foo} a
 */
function f(a) {}
```

```expect
(none)
```

### case: a DOM type with no environment

```js
/**
 * @param {HTMLElement} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-no-undefined-types
```

### case: a DOM type with the browser environment

```js options={"env":["browser"]}
/**
 * @param {HTMLElement} a
 */
function f(a) {}
```

```expect
(none)
```
