# js/promise-spec-only — cases

### case: map and delay are rewritten

```js
Promise.map(xs, f);
Promise.delay(100);
```

```expect
1:1 js/promise-spec-only
2:1 js/promise-spec-only
```

```fixed
Promise.all(xs.map(f));
new Promise((resolve) => setTimeout(resolve, 100));
```
