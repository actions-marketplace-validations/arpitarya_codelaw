# js/jsdoc-check-values — cases

### case: an unknown kind

```js
/**
 * The API.
 * @kind thing
 */
const api = {};
```

```expect
1:1 js/jsdoc-check-values
```

### case: a version with a pre-release and a v

```js
/** @version v3.0.0-beta.1 */
const api = {};
```

```expect

```
