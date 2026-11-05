# testing/playwright-valid-test-tags — cases

### case: a list

```ts
test("t", { tag: ["@a", "b"] }, async () => {});
```

```expect
1:26 testing/playwright-valid-test-tags
```

```fixed
test("t", { tag: ["@a", "@b"] }, async () => {});
```
