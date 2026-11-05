# testing/prefer-called-exactly-once-with — cases

### case: the other order

```ts
expect(f).toHaveBeenCalledWith(1);
expect(f).toHaveBeenCalledOnce();
```

```expect
1:1 testing/prefer-called-exactly-once-with
```

```fixed
expect(f).toHaveBeenCalledExactlyOnceWith(1);
```

### case: two mocks

```ts
expect(f).toHaveBeenCalledOnce();
expect(g).toHaveBeenCalledWith(1);
```

```expect
```
