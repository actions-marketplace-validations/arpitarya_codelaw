# js/array-callback-return — cases

### case: a nested function's return does not count

```js
const fns = items.map((item) => {
  function inner() {
    return item;
  }
});
```

```expect
1:13 js/array-callback-return
```

### case: forEach is not checked

```js
items.forEach((item) => {
  log(item);
});
```

```expect
(none)
```
