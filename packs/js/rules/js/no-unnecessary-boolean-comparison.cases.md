# js/no-unnecessary-boolean-comparison — cases

### case: a comparison is negated with parentheses

```js
if ((a > b) === false) swap();
```

```expect
(none)
```

### case: a value that may not be boolean stays

```js
if (flag === true) run();
```

```expect
(none)
```

### case: a comparison operand

```js
const same = a === b !== false;
```

```expect
1:14 js/no-unnecessary-boolean-comparison
```

```fixed
const same = a === b;
```
