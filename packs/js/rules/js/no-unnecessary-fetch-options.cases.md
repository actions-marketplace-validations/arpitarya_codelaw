# js/no-unnecessary-fetch-options — cases

### case: other options are kept

```js
fetch(url, { method: "GET", headers });
```

```expect
1:14 js/no-unnecessary-fetch-options
```

```fixed
fetch(url, { headers });
```
