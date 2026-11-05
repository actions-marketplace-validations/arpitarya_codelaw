# testing/prefer-spy-on — cases

### case: vitest with an implementation

```ts
Date.now = vi.fn(() => 0);
```

```expect
1:1 testing/prefer-spy-on
```

```fixed
vi.spyOn(Date, "now").mockImplementation(() => 0);
```
