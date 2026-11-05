# testing/prefer-vi-mocked — cases

### case: a generic form

```ts
const m = save as MockedFunction<typeof save>;
```

```expect
1:11 testing/prefer-vi-mocked
```

```fixed
const m = vi.mocked(save);
```
