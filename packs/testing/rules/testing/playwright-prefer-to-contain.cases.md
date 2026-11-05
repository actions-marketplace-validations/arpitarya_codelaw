# testing/playwright-prefer-to-contain — cases

### case: false

```ts
expect(a.includes(b)).toEqual(false);
```

```expect
1:1 testing/playwright-prefer-to-contain
```

```fixed
expect(a).not.toContain(b);
```

### case: includes with a start index is left alone

```ts
expect(a.includes(b, 1)).toBe(true);
```

```expect
```
