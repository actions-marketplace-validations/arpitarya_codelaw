# js/jsdoc-no-multi-asterisks — cases

### case: markdown bold is fine

```js
/**
 * **Note**: starts the server.
 */
function start() {}
```

```expect

```

### case: a doubled closing asterisk

```js
/** Starts the server. **/
function start() {}
```

```expect
1:1 js/jsdoc-no-multi-asterisks
```
