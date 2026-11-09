# testing/valid-mock-module-path — cases

### case: a package is not checked

```ts
jest.mock("some-package");
```

```expect
(none)
```

### case: doMock too

```ts
jest.doMock("./gone");
```

```expect
1:1 testing/valid-mock-module-path
```

### case: a virtual mock

```ts
jest.mock("./made-up", () => ({}), { virtual: true });
```

```expect
(none)
```

### case: not the jest object

```ts
other.mock("./missing");
```

```expect
(none)
```
