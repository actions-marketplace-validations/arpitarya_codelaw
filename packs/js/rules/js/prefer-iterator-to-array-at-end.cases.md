# js/prefer-iterator-to-array-at-end — cases

### case: filter

```js
const r = it.toArray().filter((x) => x > 1);
```

```expect
1:14 js/prefer-iterator-to-array-at-end
```

```fixed
const r = it.filter((x) => x > 1).toArray();
```

### case: flatMap

```js
const r = it.toArray().flatMap((x) => [x, x]);
```

```expect
1:14 js/prefer-iterator-to-array-at-end
```

```fixed
const r = it.flatMap((x) => [x, x]).toArray();
```

### case: a named callback

```js
const r = it.toArray().map(callback);
```

```expect
(none)
```

### case: a callback that reads the array

```js
const r = it.toArray().map((x, i, all) => all[i]);
```

```expect
(none)
```

### case: a user class

```js
class Vec {
  toArray() {
    return [1];
  }
}
const v = new Vec();
const r = v.toArray().map((x) => x);
```

```expect
(none)
```

### case: a call result is still taken for an iterator

```js
const r = makeVector().toArray().map((x) => x * 2);
```

```expect
1:24 js/prefer-iterator-to-array-at-end
```

```fixed
const r = makeVector().map((x) => x * 2).toArray();
```

### case: a function expression using arguments

```js
const r = it.toArray().map(function (x) {
  return arguments[2];
});
```

```expect
(none)
```
