# js/no-variable-usage-before-declaration — cases

### case: from a function above

```js
function f() {
  return k;
}
var k;
```

```expect
2:10 js/no-variable-usage-before-declaration
```

### case: let is not this rule's

```js
total(n);
let n = 3;
```

```expect
(none)
```
