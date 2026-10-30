# js/jsdoc-require-yields — cases

### case: a generator method

```js
class Q {
  /** Walks. */
  *walk() {
    yield this;
  }
}
```

```expect
2:3 js/jsdoc-require-yields
```

### case: a bare yield gives no value

```js
/** Pauses. */
function* pause() {
  yield;
}
```

```expect

```
