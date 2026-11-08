# js/prefer-class-fields — cases

### case: a computed value

```js
class C {
  constructor() {
    this.x = make();
  }
}
```

```expect
(none)
```

### case: after super

```js
class E extends Error {
  constructor(m) {
    super(m);
    this.name = "E";
  }
}
```

```expect
(none)
```

### case: a bare field gets the value

```js
class A {
  x;
  constructor() {
    this.x = 1;
  }
}
```

```expect
4:5 js/prefer-class-fields
```

```fixed
class A {
  x = 1;
  constructor() {
  }
}
```
