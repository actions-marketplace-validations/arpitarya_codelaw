# js/jsdoc-check-property-names — cases

### case: a repeated name

```js
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} host
 */
```

```expect
4:4 js/jsdoc-check-property-names
```

### case: the same name in two typedefs

```js
/**
 * @typedef {object} A
 * @property {string} id
 * @typedef {object} B
 * @property {string} id
 */
```

```expect
(none)
```

### case: a path with no root

```js
/**
 * @typedef {object} Options
 * @property {string} server.host
 */
```

```expect
3:4 js/jsdoc-check-property-names
```

### case: a path under the wrong root

```js
/**
 * @typedef {object} Options
 * @property {object} server
 * @property {object} client
 * @property {string} server.host
 */
```

```expect
5:4 js/jsdoc-check-property-names
```

### case: a path under its root

```js
/**
 * @typedef {object} Options
 * @property {object} server
 * @property {string} server.host
 * @property {object[]} items
 * @property {string} items[].id
 */
```

```expect
(none)
```
