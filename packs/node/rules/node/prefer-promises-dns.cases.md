# node/prefer-promises-dns — cases

### case: a local lookup function is left alone

```js
const lookup = (x, cb) => cb(null, x);
lookup(1, done);
```

```expect

```
