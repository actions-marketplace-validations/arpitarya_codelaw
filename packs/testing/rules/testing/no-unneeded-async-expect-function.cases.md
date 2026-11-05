# testing/no-unneeded-async-expect-function — cases

### case: a block body

```ts
await expect(async () => { await run(); }).rejects.toThrow();
```

```expect
1:7 testing/no-unneeded-async-expect-function
```

```fixed
await expect(run()).rejects.toThrow();
```

### case: a body that does more

```ts
await expect(async () => { a(); await run(); }).rejects.toThrow();
```

```expect
```
