# js/prefer-array-from-range — cases

### case: Array.from over new Array

```js
const indexes = Array.from(new Array(size + 1).keys());
```

```expect
1:17 js/prefer-array-from-range
```

```fixed
const indexes = Array.from({ length: size + 1 }, (_, index) => index);
```

### case: a lazy loop over keys stays

```js
for (const i of Array(count).keys()) {}
```

```expect
(none)
```

### case: values() stays

```js
const xs = [...Array(count).values()];
```

```expect
(none)
```
