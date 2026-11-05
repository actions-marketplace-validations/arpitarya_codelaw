# testing/no-commented-out-tests — cases

### case: a block comment

```ts
/* it("waits", () => {}); */
const a = 1;
```

```expect
1:1 testing/no-commented-out-tests
```

```fixed
const a = 1;
```

### case: prose that mentions a test

```ts
// the test( helper is below
```

```expect
```
