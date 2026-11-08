# js/consistent-function-scoping — cases

### case: it captures a parameter

```js
function a(k) {
  const b = () => k;
  return b;
}
```

```expect
(none)
```

### case: a callback argument

```js
function a(list) {
  return list.map((x) => x * 2);
}
```

```expect
(none)
```

### case: a returned arrow

```js
function make() {
  return (x) => x * 2;
}
```

```expect
2:10 js/consistent-function-scoping
```

### case: arrows off

```js options={"checkArrowFunctions":false}
function make() {
  return (x) => x * 2;
}
```

```expect
(none)
```
