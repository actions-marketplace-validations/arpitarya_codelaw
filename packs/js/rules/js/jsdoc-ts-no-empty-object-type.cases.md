# js/jsdoc-ts-no-empty-object-type — cases

### case: a {} default value is not a type

```js
/**
 * Merges.
 * @param {object} [opts={}] The options.
 */
function merge(opts = {}) {}
```

```expect

```

### case: inside a generic

```js
/** @type {Array<{}>} */
const rows = [];
```

```expect
1:1 js/jsdoc-ts-no-empty-object-type
```
