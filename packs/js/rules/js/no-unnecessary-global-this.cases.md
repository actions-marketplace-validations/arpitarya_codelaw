# js/no-unnecessary-global-this — cases

### case: JSON

```js
const text = globalThis.JSON.stringify(value);
```

```expect
1:14 js/no-unnecessary-global-this
```

```fixed
const text = JSON.stringify(value);
```

### case: an unknown property stays

```js
globalThis.jQuery.ajax(url);
```

```expect
(none)
```

### case: a write stays

```js
globalThis.Promise = Polyfill;
```

```expect
(none)
```

### case: an existence check stays

```js
if (globalThis.Intl) {}
```

```expect
(none)
```
