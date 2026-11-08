# js/no-mixed-completion-style — cases

### case: a hook

```js
beforeEach(async function (done) {
  await connect();
  done();
});
```

```expect
1:12 js/no-mixed-completion-style
```

### case: done alone stays

```js
it("x", (done) => {
  later(done);
});
```

```expect
(none)
```
