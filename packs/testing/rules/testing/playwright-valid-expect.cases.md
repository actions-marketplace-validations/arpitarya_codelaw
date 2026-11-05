# testing/playwright-valid-expect — cases

### case: no matcher, no call, two arguments

```ts
test("t", () => {
  expect(a);
  expect(a).toBe;
  expect(a, b, c).toBe(1);
});
```

```expect
2:3 testing/playwright-valid-expect
3:3 testing/playwright-valid-expect
4:3 testing/playwright-valid-expect
```

### case: returned, and a message argument

```ts
test("t", () => {
  expect(a, "why").toBe(1);
  return expect(p).rejects.toThrow();
});
```

```expect
```
