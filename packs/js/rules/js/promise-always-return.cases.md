# js/promise-always-return — cases

### case: an expression body returns

```js
load().then((d) => store(d)).catch(report);
```

```expect

```

### case: a throw counts

```js
load().then(function (d) {
  if (!d) throw new Error('none');
  return d;
}).catch(report);
```

```expect

```

### case: the rejection handler is not checked

```js
load().then((d) => d, (e) => {
  report(e);
});
```

```expect

```
