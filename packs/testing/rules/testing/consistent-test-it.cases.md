# testing/consistent-test-it — cases

### case: modifiers keep

```ts
it.only("a", () => {});
```

```expect
1:1 testing/consistent-test-it
```

```fixed
test.only("a", () => {});
```
