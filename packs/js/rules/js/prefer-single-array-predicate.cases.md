# js/prefer-single-array-predicate — cases

### case: every

```js
const ok = items.every(i => i.valid) && items.every(i => i.done);
```

```expect
1:12 js/prefer-single-array-predicate
```

```fixed
const ok = items.every((i) => (i.valid) && (i.done));
```

### case: different methods stay

```js
const x = a.some((v) => v > 5) && a.every((v) => v < 100);
```

```expect
(none)
```
