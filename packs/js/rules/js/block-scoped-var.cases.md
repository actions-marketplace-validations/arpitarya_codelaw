# js/block-scoped-var — cases

### case: a loop counter read after the loop

```js
for (var i = 0; i < 3; i++) {}
use(i);
```

```expect
2:5 js/block-scoped-var
```

### case: declared and used in one block

```js
function f() {
  var a = 1;
  return a;
}
```

```expect
(none)
```
