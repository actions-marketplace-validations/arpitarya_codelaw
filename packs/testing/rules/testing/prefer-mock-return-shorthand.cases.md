# testing/prefer-mock-return-shorthand — cases

### case: once

```ts
f.mockImplementationOnce(() => "a");
```

```expect
1:1 testing/prefer-mock-return-shorthand
```

```fixed
f.mockReturnValueOnce("a");
```

### case: an object is built per call

```ts
f.mockImplementation(() => ({ a: 1 }));
f.mockImplementation(() => value);
```

```expect
```
