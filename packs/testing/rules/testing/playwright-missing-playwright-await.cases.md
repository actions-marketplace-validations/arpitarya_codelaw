# testing/playwright-missing-playwright-await — cases

### case: a step

```ts
test("t", async () => {
  test.step("open", async () => {});
});
```

```expect
2:3 testing/playwright-missing-playwright-await
```

```fixed
test("t", async () => {
  await test.step("open", async () => {});
});
```

### case: a sync matcher

```ts
test("t", async () => {
  expect(1).toBe(1);
});
```

```expect
```
