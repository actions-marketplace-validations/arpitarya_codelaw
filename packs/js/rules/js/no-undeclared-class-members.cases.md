# js/no-undeclared-class-members — cases

### case: set in the constructor

```js
class A {
  constructor() {
    this.n = 1;
  }
  f() {
    return this.n;
  }
}
```

```expect
(none)
```

### case: a subclass may inherit it

```js
class A extends B {
  f() {
    return this.n;
  }
}
```

```expect
(none)
```

### case: a read of a typo

```js
class A {
  count = 0;
  f() {
    return this.cuont;
  }
}
```

```expect
4:12 js/no-undeclared-class-members
```

### case: a parameter property

```ts
class A {
  constructor(private n: number) {}
  f() {
    return this.n;
  }
}
```

```expect
(none)
```
