### case: a fill with a start index is rewritten by hand

```js
const row = Array.from({ length: n }).fill(0, 1);
```

```expect
1:13 js/no-array-from-fill
```
