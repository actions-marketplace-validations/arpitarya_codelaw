# js/no-parameters-only-used-in-recursion — cases

### case: used in the base case

```js
function f(n, acc) {
  if (n === 0) return acc;
  return f(n - 1, acc * n);
}
```

```expect
(none)
```

### case: a method

```js
class A {
  count(n, step) {
    if (n > 3) return n;
    return this.count(n + 1, step);
  }
}
```

```expect
2:12 js/no-parameters-only-used-in-recursion
```

### case: forwarded with a default

```js
function f(n, acc) {
  return n ? f(n - 1, acc || 0) : 1;
}
```

```expect
1:15 js/no-parameters-only-used-in-recursion
```
