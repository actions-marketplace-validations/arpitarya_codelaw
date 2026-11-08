# js/no-unsafe-promise-all-settled-values — cases

### case: filtered first

```js
const v = (await Promise.allSettled(ps)).filter((r) => r.status !== "rejected").map((r) => r.value);
```

```expect
(none)
```

### case: destructured in then

```js
Promise.allSettled(ps).then((rs) => rs.map(({ value }) => value));
```

```expect
1:47 js/no-unsafe-promise-all-settled-values
```

### case: an early return

```js
const rs = await Promise.allSettled(ps);
rs.map((r) => {
  if (r.status !== "fulfilled") return 0;
  return r.value;
});
```

```expect
(none)
```
