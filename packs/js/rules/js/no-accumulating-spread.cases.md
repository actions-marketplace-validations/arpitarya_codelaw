# js/no-accumulating-spread — cases

### case: an array start value is pushed onto

```js
const ids = rows.reduce((acc, r) => [...acc, r.id], []);
```

```expect
1:13 js/no-accumulating-spread
```

```fixed
const ids = rows.reduce((acc, r) => {
  acc.push(r.id);
  return acc;
}, []);
```

### case: the object form is assigned onto the accumulator

```js
const by = rows.reduce((acc, r) => ({ ...acc, [r.id]: r }), {});
```

```expect
1:12 js/no-accumulating-spread
```

```fixed
const by = rows.reduce((acc, r) => Object.assign(acc, { [r.id]: r }), {});
```
