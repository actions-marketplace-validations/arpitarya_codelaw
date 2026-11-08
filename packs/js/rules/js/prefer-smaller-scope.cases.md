# js/prefer-smaller-scope — cases

### case: used after the block

```js
function f(c) {
  let v;
  if (c) {
    v = 1;
  }
  log(v);
}
```

```expect
(none)
```

### case: a literal moved into the branch

```js
function f(b) {
  const r = 1;
  if (b) {
    log(r);
  }
}
```

```expect
2:9 js/prefer-smaller-scope
```

```fixed
function f(b) {
  if (b) {
    const r = 1;
    log(r);
  }
}
```

### case: not a literal

```js
function f(b) {
  const r = -1;
  if (b) {
    log(r);
  }
}
```

```expect
(none)
```
