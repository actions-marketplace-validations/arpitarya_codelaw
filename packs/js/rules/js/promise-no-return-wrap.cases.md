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

### case: each wrapped form is unwrapped

```js
a.then((v) => {
  return Promise.reject(v);
});
b.then((v) => Promise.resolve(v));
c.catch((e) => Promise.reject(e));
```

```expect
2:3 js/promise-no-return-wrap
4:15 js/promise-no-return-wrap
5:16 js/promise-no-return-wrap
```

```fixed
a.then((v) => {
  throw v;
});
b.then((v) => v);
c.catch((e) => { throw e; });
```
