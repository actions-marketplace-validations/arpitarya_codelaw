# js/no-extra-arguments — cases

### case: reads arguments

```js
function v() {
  return arguments.length;
}
v(1, 2);
```

```expect
(none)
```

### case: a stub

```js
const stub = () => {};
stub(1, 2);
```

```expect
(none)
```

### case: an arrow

```js
const tick = () => {
  step();
};
tick(5);
```

```expect
4:1 js/no-extra-arguments
```
