# js/no-invalid-this — cases

### case: a constructor

```js
function Counter() {
  this.count = 0;
}
```

```expect
(none)
```

### case: assigned to a property

```js
obj.reset = function () {
  this.count = 0;
};
```

```expect
(none)
```

### case: with a thisArg

```js
list.forEach(function (x) {
  this.add(x);
}, set);
```

```expect
(none)
```

### case: at the top level

```js
const self = this;
```

```expect
1:14 js/no-invalid-this
```

### case: call with undefined

```js
(function () {
  return this;
}).call(undefined);
```

```expect
2:10 js/no-invalid-this
```
