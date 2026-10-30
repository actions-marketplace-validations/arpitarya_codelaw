# js/jsdoc-check-types — cases

### case: the dotted generic

```js
/** @type {Array.<string>} */
let names = [];
```

```expect
1:1 js/jsdoc-check-types
```

### case: Object with type arguments and a link to String are fine

```js
/**
 * Maps it, like {@link String}.
 * @param {Object<string, number>} counts the counts
 */
function map(counts) {}
```

```expect

```
