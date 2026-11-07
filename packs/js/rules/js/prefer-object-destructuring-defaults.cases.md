# js/prefer-object-destructuring-defaults — cases

### case: one property

```js
const { debug } = { debug: false, ...flags };
```

```expect
1:1 js/prefer-object-destructuring-defaults
```

```fixed
const { debug = false } = { ...flags };
```

### case: a spread first stays

```js
const { debug } = { ...flags, debug: false };
```

```expect
(none)
```
