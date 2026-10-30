# js/promise-no-promise-in-callback — cases

### case: a callback without an error first is fine

```js
items.forEach((item) => {
  save(item).then(done);
});
```

```expect

```
