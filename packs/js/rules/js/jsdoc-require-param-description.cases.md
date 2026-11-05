# js/jsdoc-require-param-description — cases

### case: a description on the next line counts

```js
/**
 * Greets.
 * @param {string} name
 *   Who to greet.
 */
function greet(name) {}
```

```expect

```

### case: an optional parameter with a default

```js
/**
 * Greets.
 * @param {string} [name="you"]
 * @returns {void}
 */
function greet(name) {}
```

```expect
1:1 js/jsdoc-require-param-description
```

### case: a one-line block

```js
/** @param {number} n */
function f(n) {}
```

```expect
1:1 js/jsdoc-require-param-description
```
