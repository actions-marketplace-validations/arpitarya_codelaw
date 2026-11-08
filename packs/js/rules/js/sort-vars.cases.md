# js/sort-vars — cases

### case: literal values

```js
const z = 1, y = "a";
```

```expect
1:14 js/sort-vars
```

```fixed
const y = "a", z = 1;
```

### case: a computed value is reported without a fix

```js
let b = f(), a = g();
```

```expect
1:14 js/sort-vars
```
