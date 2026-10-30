# js/promise-valid-params — cases

### case: then with three handlers

```js
p.then(a, b, c);
```

```expect
1:1 js/promise-valid-params
```

### case: catch with none

```js
p.catch();
```

```expect
1:1 js/promise-valid-params
```

### case: the right counts are fine

```js
Promise.resolve();
Promise.reject(e);
p.then(a, b);
p.finally(done);
```

```expect

```
