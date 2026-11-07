# js/explicit-test-skip — cases

### case: Playwright

```js
test("checkout", async ({ page }) => {
  if (readOnly) {
    return;
  }
  await page.click("#buy");
});
```

```expect
2:3 js/explicit-test-skip
```

```fixed
test("checkout", async ({ page }) => {
  test.skip(readOnly);
  await page.click("#buy");
});
```

### case: a return later in the test stays

```js
it("x", function () {
  run();
  if (done) {
    return;
  }
});
```

```expect
(none)
```
