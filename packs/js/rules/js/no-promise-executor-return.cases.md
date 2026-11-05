# js/no-promise-executor-return — cases

### case: a returned resolve call becomes a statement

```js
new Promise((resolve) => {
  return resolve(1);
});
```

```expect
2:3 js/no-promise-executor-return
```

```fixed
new Promise((resolve) => {
  resolve(1); return;
});
```

### case: another returned value is outside the auto subset

```js
new Promise((resolve) => {
  return compute();
});
```

```expect
2:3 js/no-promise-executor-return
```
