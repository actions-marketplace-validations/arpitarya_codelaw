# js/prefer-early-return — cases

### case: not the last statement

```js
function f() {
  if (ok) {
    a();
    b();
  }
  done();
}
```

```expect
(none)
```

### case: a comparison is flipped

```js
function f(x) {
  if (x === 1) {
    a();
    b();
  }
}
```

```expect
2:3 js/prefer-early-return
```

```fixed
function f(x) {
  if (x !== 1) {
    return;
  }

  a();
  b();
}
```
