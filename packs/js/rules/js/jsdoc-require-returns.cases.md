# js/jsdoc-require-returns — cases

### case: a bare return is not a value

```js
/** Stops early. */
function stop() {
  return;
}
```

```expect

```

### case: a return in a nested function does not count

```js
/** Registers. */
function register() {
  on("x", function () {
    return 1;
  });
}
```

```expect

```

### case: an arrow with an expression body returns

```js
/** Doubles. */
const double = (n) => n * 2;
```

```expect
1:1 js/jsdoc-require-returns
```
