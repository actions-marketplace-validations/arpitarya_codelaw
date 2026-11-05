# testing/prefer-jest-mocked — cases

### case: a generic form

```ts
const m = save as jest.MockedFunction<typeof save>;
```

```expect
1:11 testing/prefer-jest-mocked
```

```fixed
const m = jest.mocked(save);
```

### case: another type

```ts
const m = save as Mock;
```

```expect
```
