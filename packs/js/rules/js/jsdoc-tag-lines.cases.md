# js/jsdoc-tag-lines — cases

### case: a blank line between tags

```js
/**
 * @param a first
 *
 * @returns {number}
 */
function f(a) {}
```

```expect
3:1 js/jsdoc-tag-lines
```

```fixed
/**
 * @param a first
 * @returns {number}
 */
function f(a) {}
```

### case: a blank line after the description

```js
/**
 * Adds.
 *
 * @param a
 */
function f(a) {}
```

```expect
3:1 js/jsdoc-tag-lines
```

```fixed
/**
 * Adds.
 * @param a
 */
function f(a) {}
```

### case: a blank line before the closing line

```js
/**
 * @param a
 *
 */
function f(a) {}
```

```expect
3:1 js/jsdoc-tag-lines
```

```fixed
/**
 * @param a
 */
function f(a) {}
```

### case: several blank lines go together

```js
/**
 * @param a
 *
 *
 * @returns {number}
 */
function f(a) {}
```

```expect
3:1 js/jsdoc-tag-lines
```

```fixed
/**
 * @param a
 * @returns {number}
 */
function f(a) {}
```

### case: a blank line inside an example stays

```js
/**
 * @example
 * a();
 *
 * b();
 */
function f() {}
```

```expect
(none)
```

### case: blank lines in a block with no tags stay

```js
/**
 * Adds.
 *
 * More about adding.
 */
function f() {}
```

```expect
(none)
```

### case: a one-line block

```js
/** @param a */
function f(a) {}
```

```expect
(none)
```
