# js/jsdoc-check-examples — cases

### case: an example that parses

```js
/**
 * @example
 * add(1, 2);
 */
function add() {}
```

```expect
(none)
```

### case: an example that does not parse

```js
/**
 * @example
 * add(1,
 */
function add() {}
```

```expect
3:4 js/jsdoc-check-examples
```

### case: the code after the tag on its own line

```js
/** @example add(1, */
function add() {}
```

```expect
1:14 js/jsdoc-check-examples
```

### case: a caption is not code

```js
/**
 * @example <caption>Adding</caption>
 * add(1, 2);
 */
function add() {}
```

```expect
(none)
```

### case: a fenced example is checked in its fence's language

```ts
/**
 * @example
 * ```json
 * {"a": }
 * ```
 */
function f() {}
```

```expect
4:4 js/jsdoc-check-examples
```

### case: the prose around a fence is not code

```ts
/**
 * @example
 * Use it like this:
 * ```ts
 * add(1, 2);
 * ```
 * and that is all
 */
function add() {}
```

```expect
(none)
```

### case: ignore

```js options={"ignore":["[.][.][.]"]}
/**
 * @example
 * add(...);
 */
function add() {}
```

```expect
(none)
```

### case: a comment that is not a doc comment, and a tag that is not an example

```js
/* @example add(1, */
// @example add(1,
/**
 * @examples are not one: add(1,
 * @see add(1,
 */
function add() {}
```

```expect
(none)
```

### case: two examples, one bad

```js
/**
 * @example
 * add(1, 2);
 * @example
 * add(3,
 */
function add() {}
```

```expect
5:4 js/jsdoc-check-examples
```
