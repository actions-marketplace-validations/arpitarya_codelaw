# js/function-inside-loop — cases

### case: a map callback

```js
for (var k = 0; k < n; k++) {
  list.map(function () {
    return k;
  });
}
```

```expect
(none)
```

### case: a condition that assigns

```js
var v;
while ((v = next())) {
  hs.push(() => v);
}
```

```expect
3:14 js/function-inside-loop
```
