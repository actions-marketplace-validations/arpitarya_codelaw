# testing/prefer-to-have-been-called — cases

### case: negated

```ts
expect(f).not.toHaveBeenCalledTimes(0);
```

```expect
1:1 testing/prefer-to-have-been-called
```

```fixed
expect(f).toHaveBeenCalled();
```

### case: one call

```ts
expect(f).toHaveBeenCalledTimes(1);
```

```expect
```
