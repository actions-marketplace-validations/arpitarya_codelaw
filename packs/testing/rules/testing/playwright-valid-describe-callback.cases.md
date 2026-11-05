# testing/playwright-valid-describe-callback — cases

### case: no callback

```ts
describe("api");
```

```expect
1:1 testing/playwright-valid-describe-callback
```

### case: each takes parameters

```ts
describe.each([1])("%i", (n) => {});
```

```expect
```
