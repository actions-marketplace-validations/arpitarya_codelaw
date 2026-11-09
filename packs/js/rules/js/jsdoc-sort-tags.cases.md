# js/jsdoc-sort-tags — cases

### case: returns before param

```js
/**
 * @returns {number}
 * @param {number} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-sort-tags
```

```fixed
/**
 * @param {number} a
 * @returns {number}
 */
function f(a) {}
```

### case: a continuation line moves with its tag

```js
/**
 * @returns {number} the sum,
 *   always positive
 * @param {number} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-sort-tags
```

```fixed
/**
 * @param {number} a
 * @returns {number} the sum,
 *   always positive
 */
function f(a) {}
```

### case: in order

```js
/**
 * @param {number} a
 * @returns {number}
 * @see other
 * @deprecated
 */
function f(a) {}
```

```expect
(none)
```

### case: an unlisted tag sorts after the listed ones and before @see

```js
/**
 * @param {number} a
 * @custom x
 * @see other
 */
function f(a) {}
```

```expect
(none)
```

### case: a blank line between tags

```js
/**
 * @param {number} a
 *
 * @returns {number}
 */
function f(a) {}
```

```expect
3:1 js/jsdoc-sort-tags
```

```fixed
/**
 * @param {number} a
 * @returns {number}
 */
function f(a) {}
```

### case: the closing line shares the last tag's line

```js
/**
 * @returns {number}
 * @param {number} a */
function f(a) {}
```

```expect
2:4 js/jsdoc-sort-tags
```

### case: one tag

```js
/** @returns {number} */
function f() {}
```

```expect
(none)
```

### case: several typedefs keep their properties

```js
/**
 * @typedef {object} A
 * @property {string} a
 * @typedef {object} B
 * @property {string} b
 */
```

```expect
3:4 js/jsdoc-sort-tags
```
