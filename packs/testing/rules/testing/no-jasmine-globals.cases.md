# testing/no-jasmine-globals — cases

### case: spyOn and fail are reported

```ts
spyOn(api, "get");
fail("no");
jasmine.createSpy();
```

```expect
1:1 testing/no-jasmine-globals
2:1 testing/no-jasmine-globals
3:1 testing/no-jasmine-globals
```

### case: expect.any is fine

```ts
expect(a).toEqual(expect.any(Number));
```

```expect
```
