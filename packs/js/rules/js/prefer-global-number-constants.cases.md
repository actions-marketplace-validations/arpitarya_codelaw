# js/prefer-global-number-constants — cases

### case: NEGATIVE_INFINITY stays

```js
const low = Number.NEGATIVE_INFINITY;
```

```expect
(none)
```

### case: a file that declares NaN stays

```js
function f(NaN) { return Number.NaN; }
```

```expect
(none)
```
