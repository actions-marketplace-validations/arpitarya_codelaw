### case: a name already in the file is renamed by hand

```js
const resolve = 1;
const p = new Promise((ok) => setTimeout(ok, resolve));
```

```expect
2:24 js/promise-param-names
```
