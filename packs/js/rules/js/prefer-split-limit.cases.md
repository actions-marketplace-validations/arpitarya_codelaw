# js/prefer-split-limit — cases

### case: destructuring one name

```js
const [head] = line.split(" ");
```

```expect
1:1 js/prefer-split-limit
```

```fixed
const [head] = line.split(" ", 1);
```
