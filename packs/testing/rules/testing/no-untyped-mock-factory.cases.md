# testing/no-untyped-mock-factory — cases

### case: already typed

```ts
jest.mock<typeof import("./a")>("./a", () => ({}));
jest.mock("./b");
```

```expect
```
