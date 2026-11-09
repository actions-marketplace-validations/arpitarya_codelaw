# testing/playwright-no-unsafe-references — cases

### case: a parameter is not an outer variable

```ts
test("a", async ({ page }) => {
  await page.evaluate((id) => document.getElementById(id), "main");
});
```

```expect
(none)
```

### case: addInitScript, two outer variables

```ts
test("a", async ({ page }) => {
  const a = 1;
  const b = 2;
  await page.addInitScript(() => {
    window.x = a + b;
  });
});
```

```expect
5:16 testing/playwright-no-unsafe-references
5:20 testing/playwright-no-unsafe-references
```
