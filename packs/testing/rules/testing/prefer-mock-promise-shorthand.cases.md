# testing/prefer-mock-promise-shorthand — cases

### case: return value once, rejected

```ts
f.mockReturnValueOnce(Promise.reject(err));
```

```expect
1:1 testing/prefer-mock-promise-shorthand
```

```fixed
f.mockRejectedValueOnce(err);
```

### case: an implementation that does more

```ts
f.mockImplementation((x) => Promise.resolve(x));
```

```expect
```
