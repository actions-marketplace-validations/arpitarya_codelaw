# testing/no-test-prefixes — cases

### case: describe

```ts
xdescribe("cart", () => {});
```

```expect
1:1 testing/no-test-prefixes
```

```fixed
describe.skip("cart", () => {});
```

### case: each on a prefix

```ts
xit.each([1])("n", () => {});
```

```expect
1:1 testing/no-test-prefixes
```

```fixed
it.skip.each([1])("n", () => {});
```
