# testing/playwright-prefer-native-locators — cases

### case: a test id

```ts
page.locator("[data-testid=submit]");
```

```expect
1:1 testing/playwright-prefer-native-locators
```

```fixed
page.getByTestId("submit");
```

### case: a compound selector is left

```ts
page.locator('[role=button][name="x"]');
```

```expect
(none)
```

### case: options keep the call

```ts
page.locator('[alt="logo"]', { hasText: "x" });
```

```expect
1:1 testing/playwright-prefer-native-locators
```
