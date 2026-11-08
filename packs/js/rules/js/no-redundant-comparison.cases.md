# js/no-redundant-comparison — cases

### case: different variables

```js
const ok = x > 0 && y < 0 && x === y;
```

```expect
(none)
```

### case: or is not and

```js
const ok = x > 0 || x > 1;
```

```expect
(none)
```
