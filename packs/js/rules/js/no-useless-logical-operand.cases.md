# js/no-useless-logical-operand — cases

### case: false ||

```js
const value = false || fallback;
```

```expect
1:15 js/no-useless-logical-operand
```

```fixed
const value = fallback;
```

### case: false && is false

```js
const value = false && input;
```

```expect
1:15 js/no-useless-logical-operand
```

```fixed
const value = false;
```

### case: a trailing && true in a condition

```js
if (ready && true) {}
```

```expect
1:5 js/no-useless-logical-operand
```

```fixed
if (ready) {}
```

### case: a trailing && true outside a condition stays

```js
const value = ready && true;
```

```expect
(none)
```

### case: an absorbing literal after a value stays

```js
const value = ready || true;
```

```expect
(none)
```
