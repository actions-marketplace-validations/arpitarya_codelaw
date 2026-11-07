### case: an object literal is wrapped in parentheses

```js
const rows = Array(n).fill({});
```

```expect
1:14 js/no-array-fill-with-reference-type
```

```fixed
const rows = Array.from({ length: n }, () => ({}));
```

### case: a fill without its length is rewritten by hand

```js
items.fill([]);
```

```expect
1:1 js/no-array-fill-with-reference-type
```
