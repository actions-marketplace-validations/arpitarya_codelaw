# js/jsdoc-check-access — cases

### case: two access tags in one block

```js
/**
 * Reads it.
 * @private
 * @access public
 */
function read() {}
```

```expect
1:1 js/jsdoc-check-access
```

### case: one shorthand tag is fine

```js
/** @protected */
function read() {}
```

```expect

```

### case: a plain block comment is not JSDoc

```js
/* @access internal */
function read() {}
```

```expect

```
