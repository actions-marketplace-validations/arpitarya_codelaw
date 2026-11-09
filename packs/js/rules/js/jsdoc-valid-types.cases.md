# js/jsdoc-valid-types — cases

### case: an unclosed generic

```js
/**
 * @param {Array<string} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: an unclosed array

```js
/**
 * @returns {string[}
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: a dangling union

```js
/**
 * @param {string|} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: types of every flavour

```js
/**
 * @param {Array.<string>} a
 * @param {function(string): number} b
 * @param {?string} c
 * @param {!Object} d
 * @param {string=} e
 * @param {...number} f
 * @param {{a: string, b?: number}} g
 * @param {import('x').Y} h
 * @param {(string|number)[]} i
 * @param {Map<string, Array<number>>} j
 * @param {typeof import('x')} k
 * @param {keyof T} l
 * @param {T extends U ? X : Y} m
 * @param {[a: string, b?: number]} n
 * @param {(a: string) => void} o
 * @param {*} p
 * @param {'x' | 'y'} q
 */
function f(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q) {}
```

```expect
(none)
```

### case: a callback with no name

```js
/**
 * @callback
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: @type with no type

```js
/**
 * @type
 */
const x = 1;
```

```expect
2:4 js/jsdoc-valid-types
```

### case: a name on a tag that takes none

```js
/**
 * @internal for tests
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: a type on a tag that takes none

```js
/**
 * @private {string}
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: @borrows without as

```js
/**
 * @borrows foo
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: @borrows with as

```js
/**
 * @borrows foo as bar
 */
function f() {}
```

```expect
(none)
```

### case: an empty inline link

```js
/**
 * Use {@link} here.
 */
function f() {}
```

```expect
2:8 js/jsdoc-valid-types
```

### case: a nullable object field

```js
/**
 * @param {{a: string?}} x
 */
function f(x) {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: a bad namepath

```js
/**
 * @alias foo..bar
 */
function f() {}
```

```expect
2:4 js/jsdoc-valid-types
```

### case: good namepaths

```js
/**
 * @memberof Foo#
 * @alias module:my/mod.thing
 * @typedef {object} Opts
 */
function f() {}
```

```expect
(none)
```

### case: @returns with a link is a description

```js
/**
 * @returns {@link Foo} the foo
 */
function f() {}
```

```expect
(none)
```

### case: a param with no name is another rule's

```js
/**
 * @param {string}
 */
function f(a) {}
```

```expect
(none)
```
