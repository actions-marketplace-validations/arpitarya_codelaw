# testing/playwright-prefer-comparison-matcher — cases

### case: false keeps the meaning with not

```ts
expect(a < b).toBe(false);
```

```expect
1:1 testing/playwright-prefer-comparison-matcher
```

```fixed
expect(a).not.toBeLessThan(b);
```

### case: equality is another rule

```ts
expect(a === b).toBe(true);
```

```expect
```
