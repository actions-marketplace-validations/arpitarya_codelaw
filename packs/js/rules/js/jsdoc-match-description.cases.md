# js/jsdoc-match-description — cases

### case: missing full stop before the tags

```js
/**
 * Returns the total
 *
 * @returns {number} the total
 */
function total() {
  return 1;
}
```

```expect
1:1 js/jsdoc-match-description
```

### case: no description at all is fine

```js
/**
 * @returns {number} the total
 */
function total() {
  return 1;
}
```

```expect

```

### case: a block that ends in a link inside a sentence

```js
/** Sums it, like {@link add} does. */
function total() {}
```

```expect

```

### case: a block on a variable is not checked

```js
/** the total */
let total = 0;
```

```expect

```
