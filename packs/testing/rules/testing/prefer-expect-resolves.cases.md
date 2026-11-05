# testing/prefer-expect-resolves — cases

### case: negated

```ts
async function t() {
  expect(await p).not.toBeNull();
}
```

```expect
2:3 testing/prefer-expect-resolves
```

```fixed
async function t() {
  await expect(p).resolves.not.toBeNull();
}
```
