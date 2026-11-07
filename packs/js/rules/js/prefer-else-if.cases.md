# js/prefer-else-if — cases

### case: a first branch that returns stays

```js
function f(mode) {
  if (mode === 1) {
    return a();
  }
  if (mode === 2) {
    b();
  }
}
```

```expect
(none)
```

### case: different values stay

```js
if (a === 1) {
  one();
}
if (b === 2) {
  two();
}
```

```expect
(none)
```
