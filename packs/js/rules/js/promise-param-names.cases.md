# js/promise-param-names — cases

### case: swapped names are reported on each

```js
new Promise(function (reject, resolve) {});
```

```expect
1:23 js/promise-param-names
1:31 js/promise-param-names
```

### case: a leading underscore is allowed

```js
new Promise((_resolve, _reject) => {});
```

```expect

```

### case: a single parameter without parentheses

```js
new Promise(done => done(1));
```

```expect
1:13 js/promise-param-names
```

### case: another constructor is not a promise

```js
new Task((ok, fail) => {});
```

```expect

```
