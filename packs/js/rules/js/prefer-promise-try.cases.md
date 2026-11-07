# js/prefer-promise-try — cases

### case: more work in the executor stays

```js
const p = new Promise((resolve) => {
  resolve(load());
  done();
});
```

```expect
(none)
```
