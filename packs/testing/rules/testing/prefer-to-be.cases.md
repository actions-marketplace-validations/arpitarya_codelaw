# testing/prefer-to-be — cases

### case: undefined and NaN

```ts
expect(a).toEqual(undefined);
expect(b).not.toBe(NaN);
```

```expect
1:1 testing/prefer-to-be
2:1 testing/prefer-to-be
```

```fixed
expect(a).toBeUndefined();
expect(b).not.toBeNaN();
```

### case: objects keep toEqual

```ts
expect(a).toEqual({ x: 1 });
expect(a).toBe(1);
```

```expect
```
