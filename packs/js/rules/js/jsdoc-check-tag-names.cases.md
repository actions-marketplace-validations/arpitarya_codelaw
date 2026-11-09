# js/jsdoc-check-tag-names — cases

### case: an alias is rewritten

```js
/**
 * @return {string} the name
 */
function f() {}
```

```expect
2:4 js/jsdoc-check-tag-names
```

```fixed
/**
 * @returns {string} the name
 */
function f() {}
```

### case: an alias in a one-line block

```js
/** @arg {string} a */
function f(a) {}
```

```expect
1:5 js/jsdoc-check-tag-names
```

```fixed
/** @param {string} a */
function f(a) {}
```

### case: an unknown tag has no fix

```js
/**
 * @foo bar
 */
function f() {}
```

```expect
2:4 js/jsdoc-check-tag-names
```

### case: a TypeScript tag is known

```js
/**
 * @template T
 * @satisfies {Foo}
 * @overload
 */
function f() {}
```

```expect
(none)
```

### case: an inline tag the plugin does not know

```js
/**
 * See {@code x}.
 */
function f() {}
```

```expect
2:8 js/jsdoc-check-tag-names
```

### case: an inline link is known

```js
/**
 * See {@link x} and {@linkplain y}.
 */
function f() {}
```

```expect
(none)
```

### case: a tag inside a code fence is text

```js
/**
 * Example:
 * ```
 * @foo
 * ```
 */
function f() {}
```

```expect
(none)
```

### case: a block opened with three asterisks is not JSDoc

```js
/*** @foo */
function f() {}
```

```expect
(none)
```

### case: defined_tags adds a tag

```js options={"defined_tags":["note"]}
/**
 * @note hi
 */
function f() {}
```

```expect
(none)
```
