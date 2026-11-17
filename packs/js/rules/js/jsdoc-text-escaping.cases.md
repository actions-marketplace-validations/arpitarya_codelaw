# js/jsdoc-text-escaping — cases

### case: silent without the option

```js
/** Returns Array<string> of names. */
function names() {}
```

```expect
(none)
```

### case: HTML in the description

```js options={"escapeHTML":true}
/** Returns Array<string> & more. */
function names() {}
```

```expect
1:1 js/jsdoc-text-escaping
```

```fixed
/** Returns Array&lt;string> &amp; more. */
function names() {}
```

### case: an entity and a lone ampersand are left alone

```js options={"escapeHTML":true}
/** Fish &amp; chips & more, a < b. */
function names() {}
```

```expect
(none)
```

### case: a tag's description

```js options={"escapeHTML":true}
/**
 * Names.
 * @returns {Array} The Array<string>.
 */
function names() {}
```

```expect
3:4 js/jsdoc-text-escaping
```

```fixed
/**
 * Names.
 * @returns {Array} The Array&lt;string>.
 */
function names() {}
```

### case: an example is not read

```js options={"escapeHTML":true}
/**
 * Names.
 * @example
 * const a = b < c;
 */
function names() {}
```

```expect
(none)
```

### case: Markdown

```js options={"escapeMarkdown":true}
/** Use `code` here, not ``two`` or `unpaired. */
function names() {}
```

```expect
1:1 js/jsdoc-text-escaping
```

```fixed
/** Use \`code` here, not \``two`` or `unpaired. */
function names() {}
```
