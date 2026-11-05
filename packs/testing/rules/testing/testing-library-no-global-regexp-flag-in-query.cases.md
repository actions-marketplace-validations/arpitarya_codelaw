# testing/testing-library-no-global-regexp-flag-in-query — cases

### case: only g

```ts
screen.getByText(/save/g);
```

```expect
1:24 testing/testing-library-no-global-regexp-flag-in-query
```

```fixed
screen.getByText(/save/);
```
