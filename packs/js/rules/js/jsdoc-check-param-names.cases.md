# js/jsdoc-check-param-names — cases

### case: a misspelt name

```js
/**
 * @param {string} nme
 */
function greet(name) {}
```

```expect
2:4 js/jsdoc-check-param-names
```

### case: a tag for a parameter that is not there

```js
/**
 * @param {string} a
 * @param {string} b
 */
function f(a) {}
```

```expect
3:4 js/jsdoc-check-param-names
```

### case: a duplicate tag

```js
/**
 * @param {string} a
 * @param {string} a
 */
function f(a) {}
```

```expect
3:4 js/jsdoc-check-param-names
```

### case: names in the wrong order

```js
/**
 * @param {string} b
 * @param {string} a
 */
function f(a, b) {}
```

```expect
2:4 js/jsdoc-check-param-names
```

### case: a destructured parameter takes any root name

```js
/**
 * @param {object} opts
 * @param {string} opts.name
 */
function f({ name }) {}
```

```expect
(none)
```

### case: a path before its root

```js
/**
 * @param {string} opts.name
 * @param {object} opts
 */
function f(opts) {}
```

```expect
2:4 js/jsdoc-check-param-names
```

### case: a path under another parameter

```js
/**
 * @param {object} a
 * @param {object} b
 * @param {string} a.x
 */
function f(a, b) {}
```

```expect
4:4 js/jsdoc-check-param-names
```

### case: an arrow function in a variable

```js
/**
 * @param {number} x
 */
const f = (y) => y;
```

```expect
2:4 js/jsdoc-check-param-names
```

### case: a class method

```js
class K {
  /**
   * @param {number} x
   */
  m(y) {}
}
```

```expect
3:6 js/jsdoc-check-param-names
```

### case: an optional name and a default

```js
/**
 * @param {number} [a]
 * @param {number} [b=2]
 */
function f(a, b = 1) {}
```

```expect
(none)
```

### case: a rest parameter

```js
/**
 * @param {...number} nums
 */
function f(...nums) {}
```

```expect
(none)
```

### case: a block that does not document a function

```js
/**
 * @param {string} nme
 */
const value = 1;
```

```expect
(none)
```

### case: allow_extra_trailing_param_docs

```js options={"allow_extra_trailing_param_docs":true}
/**
 * @param {string} a
 * @param {string} b
 */
function f(a) {}
```

```expect
(none)
```
