# node/callback-return — cases

### case: the last statement of the function is left alone

```js
function done(cb) {
  work();
  cb(null);
}
```

```expect

```

### case: a call followed by return is left alone

```js
function f(x, next) {
  if (x) {
    next();
    return;
  }
}
```

```expect

```

### case: a braceless if is reported

```js
function f(err, cb) {
  if (err) cb(err);
  cb(null, 1);
}
```

```expect
2:12 node/callback-return
```

### case: a call outside any function is left alone

```js
if (a) {
  cb();
}
```

```expect

```
