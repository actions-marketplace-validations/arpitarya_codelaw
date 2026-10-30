# js/jsdoc-multiline-blocks — cases

### case: only the last line has text

```js
/**
 * Adds. */
function add() {}
```

```expect
1:1 js/jsdoc-multiline-blocks
```

### case: a single-line block

```js
/** Adds. */
function add() {}
```

```expect

```
