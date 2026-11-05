# testing/prefer-to-have-been-called-times — cases

### case: negated

```ts
expect(f.mock.calls).not.toHaveLength(2);
```

```expect
1:1 testing/prefer-to-have-been-called-times
```

```fixed
expect(f).not.toHaveBeenCalledTimes(2);
```
