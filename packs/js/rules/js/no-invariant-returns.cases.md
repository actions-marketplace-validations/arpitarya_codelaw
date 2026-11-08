# js/no-invariant-returns — cases

### case: guard and log

```js
function g(x) {
  if (!x) {
    warn();
    return true;
  }
  work();
  return true;
}
```

```expect
(none)
```

### case: a const

```js
function g(x) {
  const r = "ok";
  if (x > 1) {
    return r;
  }
  return "ok";
}
```

```expect
1:10 js/no-invariant-returns
```
