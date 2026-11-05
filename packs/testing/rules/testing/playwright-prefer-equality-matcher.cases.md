# testing/playwright-prefer-equality-matcher — cases

### case: not equal

```ts
expect(a !== b).toBe(true);
```

```expect
1:1 testing/playwright-prefer-equality-matcher
```

```fixed
expect(a).not.toBe(b);
```
