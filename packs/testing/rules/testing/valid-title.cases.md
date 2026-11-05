# testing/valid-title — cases

### case: empty, a number and a repeated prefix

```ts
test("", () => {});
describe(1, () => {});
test("test adds", () => {});
```

```expect
1:6 testing/valid-title
2:10 testing/valid-title
3:7 testing/valid-title
```

### case: a variable title is fine

```ts
test(name, () => {});
```

```expect
```
