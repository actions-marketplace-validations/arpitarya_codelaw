# js/jsdoc-require-rejects — cases

### case: a throw in a nested callback does not count

```js
/** Runs. */
async function run() {
  items.forEach(() => {
    throw new Error("x");
  });
}
```

```expect

```

### case: a sync function returning Promise.reject

```js
/** Fails. */
function fail() {
  return Promise.reject(new Error("no"));
}
```

```expect
1:1 js/jsdoc-require-rejects
```
