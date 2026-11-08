# js/prefer-number-coercion — cases

### case: parseInt with radix 10

```js
const n = parseInt(input, 10);
```

```expect
1:11 js/prefer-number-coercion
```

```fixed
const n = Math.trunc(Number(input));
```

### case: another radix stays

```js
const n = parseInt(input, 16);
```

```expect
(none)
```

### case: parseInt without a radix stays

```js
const n = parseInt(input);
```

```expect
(none)
```
