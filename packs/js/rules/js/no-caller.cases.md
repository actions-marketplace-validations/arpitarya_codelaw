### case: a named function refers to itself by its name

```js
function fact(n) { return n < 2 ? 1 : n * arguments.callee(n - 1); }
```

```expect
1:43 js/no-caller
```

```fixed
function fact(n) { return n < 2 ? 1 : n * fact(n - 1); }
```
