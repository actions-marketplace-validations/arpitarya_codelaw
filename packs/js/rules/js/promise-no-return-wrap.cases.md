# js/promise-no-return-wrap — cases

### case: an arrow body

```js
p.catch((e) => Promise.reject(e));
```

```expect
1:16 js/promise-no-return-wrap
```

### case: outside a callback it is fine

```js
function f() {
  return Promise.resolve(1);
}
```

```expect

```
