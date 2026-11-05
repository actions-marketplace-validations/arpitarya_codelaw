# testing/testing-library-await-async-events — cases

### case: awaited and setup

```ts
test("t", async () => {
  const user = userEvent.setup();
  await userEvent.type(input, "a");
});
```

```expect
```
