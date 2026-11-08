### case: an async callback stays async

```js
run(async function (x) {
  await x;
});
```

```expect
1:5 js/prefer-arrow-callback
```

```fixed
run(async (x) => {
  await x;
});
```

### case: a return type is kept

```ts
items.map(function (x): number {
  return x * 2;
});
```

```expect
1:11 js/prefer-arrow-callback
```

```fixed
items.map((x): number => {
  return x * 2;
});
```
