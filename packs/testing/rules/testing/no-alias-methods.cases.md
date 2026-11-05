# testing/no-alias-methods — cases

### case: a modifier in between

```ts
expect(f).not.toBeCalled();
await expect(p).rejects.toThrowError();
```

```expect
1:1 testing/no-alias-methods
2:7 testing/no-alias-methods
```

```fixed
expect(f).not.toHaveBeenCalled();
await expect(p).rejects.toThrow();
```

### case: not an expect chain

```ts
spy.toBeCalled();
expect.toBeCalled;
```

```expect
```
