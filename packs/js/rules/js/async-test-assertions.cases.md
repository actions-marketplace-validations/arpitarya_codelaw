# js/async-test-assertions — cases

### case: an async callback

```js
test("fails", async () => {
  expect(job()).rejects.toThrow();
});
```

```expect
2:3 js/async-test-assertions
```

```fixed
test("fails", async () => {
  await expect(job()).rejects.toThrow();
});
```

### case: node's assert.rejects

```js
test("fails", () => {
  assert.rejects(job());
});
```

```expect
2:3 js/async-test-assertions
```

```fixed
test("fails", async () => {
  await assert.rejects(job());
});
```

### case: an awaited assertion stays

```js
test("fails", async () => {
  await expect(job()).rejects.toThrow();
});
```

```expect
(none)
```

### case: a done callback stays

```js
it("loads", (done) => {
  expect(load()).resolves.toBe(1);
  done();
});
```

```expect
(none)
```
