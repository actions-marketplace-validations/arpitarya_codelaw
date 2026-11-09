# js/jsdoc-normalize-see-links — cases

### case: a Markdown link

```js
/**
 * @see [Docs](https://example.com)
 */
function f() {}
```

```expect
2:4 js/jsdoc-normalize-see-links
```

```fixed
/**
 * @see {@link https://example.com|Docs}
 */
function f() {}
```

### case: a prefix link

```js
/**
 * @see [Docs]{@link https://example.com}
 */
function f() {}
```

```expect
2:4 js/jsdoc-normalize-see-links
```

```fixed
/**
 * @see {@link https://example.com|Docs}
 */
function f() {}
```

### case: already the pipe form

```js
/**
 * @see {@link https://example.com|Docs}
 */
function f() {}
```

```expect
(none)
```

### case: a link with no label

```js
/**
 * @see {@link https://example.com}
 */
function f() {}
```

```expect
(none)
```

### case: the space form cannot be rewritten safely

```js
/**
 * @see {@link https://example.com Docs}
 */
function f() {}
```

```expect
2:4 js/jsdoc-normalize-see-links
```

### case: a scoped package target

```js
/**
 * @see [pkg](@scope/pkg)
 */
function f() {}
```

```expect
2:4 js/jsdoc-normalize-see-links
```

### case: a link in another tag is left alone

```js
/**
 * @param {string} a [Docs](https://example.com)
 */
function f(a) {}
```

```expect
(none)
```

### case: a continuation line

```js
/**
 * @see the manual,
 *   [Docs](https://example.com)
 */
function f() {}
```

```expect
2:4 js/jsdoc-normalize-see-links
```

```fixed
/**
 * @see the manual,
 *   {@link https://example.com|Docs}
 */
function f() {}
```
