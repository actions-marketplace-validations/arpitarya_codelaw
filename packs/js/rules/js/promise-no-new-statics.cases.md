# js/promise-no-new-statics — cases

### case: every static

```js
new Promise.all([a]);
new Promise.race([a]);
```

```expect
1:1 js/promise-no-new-statics
2:1 js/promise-no-new-statics
```

```fixed
Promise.all([a]);
Promise.race([a]);
```

### case: the constructor itself is fine

```js
new Promise((resolve) => resolve(1));
```

```expect

```
