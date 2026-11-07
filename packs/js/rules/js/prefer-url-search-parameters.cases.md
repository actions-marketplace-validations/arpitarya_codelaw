# js/prefer-url-search-parameters — cases

### case: new Map

```js
const params = new Map(query.split('&').map(p => p.split('=')));
```

```expect
1:24 js/prefer-url-search-parameters
```

```fixed
const params = new Map(new URLSearchParams(query));
```

### case: a bare pipeline stays

```js
const pairs = query.split('&').map(p => p.split('='));
```

```expect
(none)
```
