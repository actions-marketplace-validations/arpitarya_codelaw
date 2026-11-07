# js/no-url-in-search-params — cases

### case: .href

```js
const params = new URLSearchParams(location.href);
```

```expect
1:16 js/no-url-in-search-params
```

```fixed
const params = new URLSearchParams(location.search);
```

### case: a query string stays

```js
const params = new URLSearchParams("?q=1");
```

```expect
(none)
```
