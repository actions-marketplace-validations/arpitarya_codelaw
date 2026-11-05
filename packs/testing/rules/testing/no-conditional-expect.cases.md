# testing/no-conditional-expect — cases

### case: catch

```ts
test("t", async () => {
  try { await run(); } catch (e) { expect(e).toBeDefined(); }
});
```

```expect
2:36 testing/no-conditional-expect
```
