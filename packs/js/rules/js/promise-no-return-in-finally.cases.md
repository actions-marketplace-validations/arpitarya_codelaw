# js/promise-no-return-in-finally — cases

### case: a bare return is fine

```js
load().finally(function () {
  if (done) return;
  cleanup();
});
```

```expect

```

### case: a return in a nested function is that function's

```js
load().finally(() => {
  items.map((x) => { return x; });
});
```

```expect

```
