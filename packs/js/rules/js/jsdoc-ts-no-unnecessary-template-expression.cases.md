# js/jsdoc-ts-no-unnecessary-template-expression — cases

### case: a string expression

```js
/**
 * @param {`a${'b'}c`} x
 */
function f(x) {}
```

```expect
2:12 js/jsdoc-ts-no-unnecessary-template-expression
```

```fixed
/**
 * @param {`abc`} x
 */
function f(x) {}
```

### case: a lone expression

```js
/**
 * @param {`${string}`} x
 */
function f(x) {}
```

```expect
2:12 js/jsdoc-ts-no-unnecessary-template-expression
```

```fixed
/**
 * @param {string} x
 */
function f(x) {}
```

### case: an expression that is needed

```js
/**
 * @param {`id-${number}`} x
 */
function f(x) {}
```

```expect
(none)
```

### case: a string expression beside another

```js
/**
 * @param {`${'a'}-${number}`} x
 */
function f(x) {}
```

```expect
2:12 js/jsdoc-ts-no-unnecessary-template-expression
```

```fixed
/**
 * @param {`a-${number}`} x
 */
function f(x) {}
```
