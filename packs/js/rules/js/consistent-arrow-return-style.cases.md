# js/consistent-arrow-return-style — cases

### case: a multi-line concise body

```js
const f = () => g(
  a,
);
```

```expect
1:17 js/consistent-arrow-return-style
```

```fixed
const f = () => {
  return g(
    a,
  );
};
```

### case: a bare return

```js
const f = () => {
  return;
};
```

```expect
(none)
```

### case: an object is wrapped

```js
const f = () => {
  return { a: 1 };
};
```

```expect
1:17 js/consistent-arrow-return-style
```

```fixed
const f = () => ({ a: 1 });
```
