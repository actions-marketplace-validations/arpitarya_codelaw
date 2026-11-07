# js/synchronous-suite-callback — cases

### case: a nested suite

```js
describe("outer", () => {
  describe("inner", async () => {
    it("x", () => {});
  });
});
```

```expect
2:21 js/synchronous-suite-callback
```

```fixed
describe("outer", () => {
  describe("inner", () => {
    it("x", () => {});
  });
});
```
