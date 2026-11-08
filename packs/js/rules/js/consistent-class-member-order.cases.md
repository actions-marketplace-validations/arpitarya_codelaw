# js/consistent-class-member-order — cases

### case: in order

```js
class A {
  static s = 1;
  #x = 1;
  y = 2;
  constructor() {}
  #m() {}
  n() {}
}
```

```expect
(none)
```
