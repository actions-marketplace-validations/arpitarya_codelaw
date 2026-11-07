# js/no-floating-point-equality — cases

### case: not equal

```js
const off = total * rate !== 0.25;
```

```expect
1:13 js/no-floating-point-equality
```

```fixed
const off = Math.abs(total * rate - 0.25) >= Number.EPSILON;
```

### case: an integer literal stays

```js
if (a + b === 3) {}
```

```expect
(none)
```
