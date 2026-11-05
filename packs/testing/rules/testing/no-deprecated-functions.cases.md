# testing/no-deprecated-functions — cases

### case: require.requireActual

```ts
const a = require.requireActual("./a");
```

```expect
1:11 testing/no-deprecated-functions
```

```fixed
const a = jest.requireActual("./a");
```
