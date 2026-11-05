# testing/playwright-prefer-to-have-length — cases

### case: negated and toEqual

```ts
expect(a.length).not.toEqual(0);
```

```expect
1:1 testing/playwright-prefer-to-have-length
```

```fixed
expect(a).not.toHaveLength(0);
```

### case: another matcher

```ts
expect(a.length).toBeGreaterThan(0);
```

```expect
```
