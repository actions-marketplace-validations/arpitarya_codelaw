# js/no-useless-recursion — cases

### case: a named function expression

```js
const g = function loop(a) {
  return loop(a - 1);
};
g(1);
```

```expect
2:10 js/no-useless-recursion
```

### case: work after the call

```js
function sum(n) {
  return n + sum(n - 1);
}
sum(1);
```

```expect
(none)
```

### case: inside try

```js
function f(v) {
  try {
    return f(v.n);
  } finally {
    done();
  }
}
f(1);
```

```expect
(none)
```

### case: a shadowing name

```js
function f(f) {
  return f(1);
}
f(g);
```

```expect
(none)
```
