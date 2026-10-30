# js/jsdoc-require-returns-check — cases

### case: @returns {void} is fine

```js
/**
 * Logs.
 * @returns {void}
 */
function log() {}
```

```expect

```

### case: an async function is skipped

```js
/**
 * Saves.
 * @returns {Promise<number>} The id.
 */
async function save() {
  await write();
}
```

```expect

```
